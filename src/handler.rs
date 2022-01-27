use tokio::time::{sleep, Duration};
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
	sleep_time: u64,
	include_only: Vec<u64>,
}

// counter for the number of present users in the guild

struct Users {
	online: u64,
	count: u64,
}

// response json structure for approximated user count from the widget api
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
			sleep_time : settings.sleep_time,
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
		guild_id: &GuildId,
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

				if self.sleep_time > 0 {
					sleep(Duration::from_millis(self.sleep_time)).await;
				}

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
}

#[async_trait]
impl EventHandler for Handler {
	// fix approximate user count when a user starts membership with the guild
	// this is necessary in case a user joins invisible so we can adjust our correct online users offset
	async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member) {
		if self.guild_id != _guild_id.0 {
			return;
		}

		let user = &_new_member.user;

		if self.skip_cycle(&user.id.0) {
			return;
		}

		let approximate_users = match self.get_approximate_user_counts(&_ctx, &_guild_id).await {
			Ok(users) => users,
			Err(e) => {
				println!("{}", format!("Could not refresh users count: {}", e).red());
				return;
			}
		};

		let approximate_online = approximate_users.0;
		let approximate_count = approximate_users.1;
		let mut current_users = self.users.lock().await;

		// if the user is invisible, still count the user as online
		current_users.online = approximate_online;

		let name = format!("{}#{}", user.name, user.discriminator);

		if approximate_count > current_users.count {
			println!(
				"{}",
				format!(
					"[JOIN] {}",
					name
				)
				.green()
			);
			current_users.count += 1;
		} else {
			println!(
				"{}",
				format!(
					"[LEFT] {}",
					name
				)
				.green()
			);
			current_users.count -= 1;
		}
	}

	// decrement the membership and online count when a user leaves the guild
	async fn guild_member_removal(&self, _ctx: Context, _guild_id: GuildId, _kicked: User) {
		if self.guild_id != _guild_id.0 {
			return;
		}

		let mut current_users = self.users.lock().await;
		current_users.online -= 1;
		current_users.count = 1;
	}

	// get the real status when a user changes theirs
	async fn presence_update(&self, _ctx: Context, _new_data: PresenceUpdateEvent) {
		let presence_guild = &_new_data.guild_id.unwrap();

		// if the guild is not the guild we are tracking, return
		if presence_guild.0 != self.guild_id {
			return;
		}

		let presence = &_new_data.presence;

		// Skip, if the user presence does not explicitly change
		// but just the status for example
		//
		// this is to prevent the bot from getting rate limited
		// due to getting the approximate user count too often for no reason
		//
		// below we repeat the checks to differentiate the status change
		if presence.status != OnlineStatus::Offline {
			if None == presence.user {
				return;
			}
		}

		// skip if the user is not in the include_only list if list not empty
		if self.skip_cycle(&presence.user_id.0) {
			return;
		}

		let mut current_users = self.users.lock().await;

		// get the approximate user counts
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

		// this is true if the user changes their status to offline
		if presence.status == OnlineStatus::Offline {
			let user = presence.user_id.to_user(&_ctx).await.unwrap();
			let name = format!("{}#{}", user.name, user.discriminator);

			// comparing the online count to the approximate online count
			// if below the approximate online count, then the user is offline
			if current_users.online > users_online {
				println!("{}", format!("[OFF] {}", name).green());
			}
			// if above the approximate online count, then the user is invisible
			else {
				println!("{}", format!("[INVISIBLE] {}", name).red());
			}
		}
		// this is true if the user changes their status to online
		else if let Some(user) = &presence.user {
			println!("{}", format!("[ON] {}#{}", user.name, user.discriminator).green());
		}

		// fix offset
		current_users.online = users_online;
	}

	// initialize the approximate user count as a starting point to calculate the user offsets
	// when a user joins or leaves or changes the online status
	async fn ready(&self, _ctx: Context, _: Ready) {
		let (users_count, users_online) = match self
			.get_approximate_user_counts(&_ctx, &GuildId(self.guild_id))
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

		let mut current_users = self.users.lock().await;
		current_users.online = users_count;
		current_users.count = users_online;

		println!("{}", format!("Up and running, ready to catch those Dinklebergs!\n\n[GUILD ID]: {}\n[ON]: {}\n[OFF]: {}\n", self.guild_id, current_users.online, current_users.count - current_users.online).yellow());
	}
}
