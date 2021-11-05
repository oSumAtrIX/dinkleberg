use crate::settings::Settings;
use colored::Colorize;
use serde::Deserialize;
use serenity::{
	async_trait,
	model::{
		event::PresenceUpdateEvent,
		gateway::Ready,
		guild::Member,
		id::GuildId,
		prelude::{OnlineStatus, User},
	},
	prelude::*,
};

pub struct Handler {
	guild_id: u64,
	users: Mutex<Users>,
	use_widget: bool,
	include_only: Vec<u64>,
}

struct Users {
	online: u64,
	count: u64,
}

#[derive(Deserialize, Debug)]
struct Widget {
	presence_count: u64,
}

impl Handler {
	pub fn new(guild_id: u64, settings: &Settings) -> Self {
		Handler {
			guild_id,
			users: Mutex::new(Users {
				online: 0,
				count: 0,
			}),
			use_widget: settings.use_widget,
			include_only: settings.include_only.clone(),
		}
	}

	// Get the number of users online in the guild
	// despite being set to invisible using the approximate presence count field
	// and the total number of users in the guild
	async fn get_approximate_user_counts(
		&self,
		ctx: &Context,
		guild_id: GuildId,
	) -> Result<(u64, u64), reqwest::Error /* replace with generic error */> {
		let guild = match guild_id.to_partial_guild_with_counts(ctx).await {
			Ok(guild) => guild,
			Err(e) => {
				println!("{}", format!("Failed to get guild: {}", e).red());
				return Ok((0, 0));
			}
		};
		let presence_count = match self.use_widget {
			true => {
				reqwest::blocking::get(&format!(
					"https://canary.discord.com/api/guilds/{}/widget.json",
					guild_id
				))?
				.json::<Widget>()?
				.presence_count
			}
			false => guild.approximate_presence_count.unwrap(),
		};

		Ok((presence_count, guild.approximate_member_count.unwrap()))
	}

	// skip the user if they are not in the include_only list
	fn skip_cycle(&self, id: &u64) -> bool {
		if self.include_only.is_empty() || self.include_only.contains(id) {
			return false;
		}
		true
	}
	// update user count and online count when a user joins or leaves the server to keep the count accurate
	async fn refresh_users_count(&self, _ctx: Context, _guild_id: GuildId, user: User) {
		if self.skip_cycle(&user.id.0) {
			return;
		}

		let approximate_users = match self.get_approximate_user_counts(&_ctx, _guild_id).await {
			Ok(users) => users,
			Err(e) => {
				println!("{}", format!("Could not refresh users count: {}", e).red());
				return;
			}
		};

		let approximate_online = approximate_users.0;
		let approximate_count = approximate_users.1;
		let mut tracked_users = self.users.lock().await;

		tracked_users.online = approximate_online;

		if approximate_count > tracked_users.count {
			println!(
				"{}",
				format!(
					"{} joined the server. Approximation might eventually cause issues at first..",
					user.name
				)
				.green()
			);
		} else {
			println!(
				"{}",
				format!(
					"{} left the server. Approximation might eventually cause issues at first..",
					user.name
				)
				.green()
			);
		}

		tracked_users.count = approximate_count;
	}
}

#[async_trait]
impl EventHandler for Handler {
	async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member) {
		self.refresh_users_count(_ctx, _guild_id, _new_member.user)
			.await;
	}

	async fn guild_member_removal(&self, _ctx: Context, _guild_id: GuildId, _kicked: User) {
		self.refresh_users_count(_ctx, _guild_id, _kicked).await;
	}

	// get the real status when a user changes theirs
	async fn presence_update(&self, _ctx: Context, _new_data: PresenceUpdateEvent) {
		let presence_guild = _new_data.guild_id.unwrap();

		if presence_guild != self.guild_id {
			return;
		}

		if self.skip_cycle(&_new_data.presence.user_id.0) {
			return;
		}

		let mut tracked_users = self.users.lock().await;

		let (users_online, _) = match self
			.get_approximate_user_counts(&_ctx, presence_guild)
			.await
		{
			Ok(users) => users,
			Err(e) => {
				println!("{}", format!("Could not get user count: {}", e).red());
				return;
			}
		};

		if let Some(user) = _new_data.presence.user {
			if users_online != tracked_users.online {
				tracked_users.online += 1;
			}

			println!(
				"{}",
				format!(
					"{} is now online, adding up to a total of {} users online",
					user.name, tracked_users.online
				)
				.green()
			);
			return;
		}

		if _new_data.presence.status == OnlineStatus::Offline {
			let user = _new_data
				.presence
				.user_id
				.to_user(&_ctx)
				.await
				.unwrap()
				.name;

			if users_online < tracked_users.online {
				// user really went offline
				println!(
					"{}",
					format!(
						"{} is now offline, adding up to a total of {} offline users",
						user,
						tracked_users.count - tracked_users.online + 1
					)
					.green()
				);
			} else {
				// user set to invisible
				println!("{}", format!("Caught {} dinkleberging!", user).red());
			}
			tracked_users.online -= 1;
		}
	}

	// initialize the approximate user count as a starting point to calculate the user offsets
	// when a user joins or leaves or changes the online status
	async fn ready(&self, _ctx: Context, _: Ready) {
		let (users_count, users_online) = match self
			.get_approximate_user_counts(&_ctx, GuildId(self.guild_id))
			.await
		{
			Ok(users) => users,
			Err(e) => {
				println!(
					"{}",
					format!("Could not initialize users count: {}", e).red()
				);
				return;
			}
		};

		let mut tracked_users = self.users.lock().await;
		tracked_users.online = users_count;
		tracked_users.count = users_online;

		println!("{}", format!("Up and running, ready to catch those Dinklebergs!\n\nTargeting guild: {}\nInitial online count: {}\nInitial offline count: {}\n", self.guild_id, tracked_users.online, tracked_users.count - tracked_users.online).yellow());
	}
}
