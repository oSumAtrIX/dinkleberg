use std::env;

use serenity::{
	async_trait,
	client::bridge::gateway::GatewayIntents,
	model::{
		event::PresenceUpdateEvent,
		gateway::Ready,
		guild::Member,
		id::GuildId,
		prelude::{OnlineStatus, User},
	},
	prelude::*,
};

struct Handler {
	guild_id: u64,
	users: Mutex<Users>,
}

struct Users {
	online: u64,
	count: u64,
}

impl Handler {
	pub fn new(guild_id: u64) -> Self {
		Handler {
			guild_id,
			users: Mutex::new(Users {
				online: 0,
				count: 0,
			}),
		}
	}

	async fn refresh_users_count(&self, _ctx: Context, _guild_id: GuildId, user: User) {
		let approximate_users = get_approximate_user_counts(&_ctx, _guild_id).await;
		let approximate_count = approximate_users[1];
		let approximate_online = approximate_users[0];
		let mut tracked_users = self.users.lock().await;

		tracked_users.online = approximate_online;

		if approximate_count > tracked_users.count {
			println!(
				"{} joined the server. Approximation might eventually cause issues at first..",
				user.name
			);
		} else {
			println!(
				"{} left the server. Approximation might eventually cause issues at first..",
				user.name
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

	async fn guild_member_removal(
		&self,
		_ctx: Context,
		_guild_id: GuildId,
		_kicked: User,
		_member_data_if_available: Option<Member>,
	) {
		self.refresh_users_count(_ctx, _guild_id, _kicked).await;
	}

	async fn presence_update(&self, _ctx: Context, _new_data: PresenceUpdateEvent) {
		let presence_guild = _new_data.guild_id.unwrap();

		if presence_guild != self.guild_id {
			return;
		}

		let mut tracked_users = self.users.lock().await;

		if let Some(user) = _new_data.presence.user {
			tracked_users.online += 1;
			println!(
				"{} came on, adding up to a total of {} users online",
				user.name, tracked_users.online
			);
			return;
		}

		if _new_data.presence.status == OnlineStatus::Offline {
			let users_count = get_approximate_user_counts(&_ctx, presence_guild).await;

			let user = _new_data
				.presence
				.user_id
				.to_user(&_ctx)
				.await
				.unwrap()
				.name;

			if users_count[0] < tracked_users.online {
				println!(
					"{} went really offline, adding to a total of {} offline users",
					user,
					tracked_users.count - tracked_users.online + 1
				);
			} else {
				println!("Catched {} dinkleberging!", user);
			}
			tracked_users.online -= 1;
		}
	}

	async fn ready(&self, _ctx: Context, _: Ready) {
		let users_count = get_approximate_user_counts(&_ctx, GuildId(self.guild_id)).await;

		let mut tracked_users = self.users.lock().await;
		tracked_users.online = users_count[0];
		tracked_users.count = users_count[1];

		println!("\nUp and running, ready to catch those Dinklebergs!\n\nTargeting guild: {}\nInitial online count: {}\nInitial offline count: {}\n", self.guild_id, tracked_users.online, tracked_users.count - tracked_users.online);
	}
}

async fn get_approximate_user_counts(ctx: &Context, guild_id: GuildId) -> [u64; 2] {
	let guild = guild_id.to_partial_guild_with_counts(ctx).await.unwrap();
	[
		guild.approximate_presence_count.unwrap(),
		guild.approximate_member_count.unwrap(),
	]
}

fn parse_u64(s: &str) -> u64 {
	match s.parse::<u64>() {
		Ok(n) => n,
		Err(_) => panic!("Could not parse {} as u64", s),
	}
}

#[tokio::main]
async fn main() {
	let token = match env::var("DISCORD_TOKEN") {
		Ok(token) => token,
		Err(_) => {
			println!("Please set the environment variable DISCORD_TOKEN");
			return;
		}
	};

	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("Usage: {} <tracking_guild_id>", args[0]);
		return;
	}

	let guild_id = parse_u64(&args[1]);
	let mut client = Client::builder(token)
		.event_handler(Handler::new(guild_id))
		.intents(GatewayIntents::GUILD_PRESENCES | GatewayIntents::GUILD_MEMBERS)
		.await
		.expect("Error creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}
