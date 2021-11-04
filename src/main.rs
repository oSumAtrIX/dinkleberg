mod handler;

#[tokio::main]
async fn main() {
	if cfg!(target_os = "windows") {
		// clear console
		std::process::Command::new("cmd")
			.arg("/c")
			.arg("cls")
			.status()
			.unwrap();

		use colored::control;

		// legacy support
		#[cfg(windows)]
		if let Err(_) = control::set_virtual_terminal(true) {
			println!("Failed to set virtual terminal")
		};
	} else {
		// clear console
		std::process::Command::new("clear").status().unwrap();
	}

	// print logo
	println!("\n{}\n", "'########::'####:'##::: ##:'##:::'##:'##:::::::'########:'########::'########:'########:::'######:::\n ##.... ##:. ##:: ###:: ##: ##::'##:: ##::::::: ##.....:: ##.... ##: ##.....:: ##.... ##:'##... ##::\n ##:::: ##:: ##:: ####: ##: ##:'##::: ##::::::: ##::::::: ##:::: ##: ##::::::: ##:::: ##: ##:::..:::\n ##:::: ##:: ##:: ## ## ##: #####:::: ##::::::: ######::: ########:: ######::: ########:: ##::'####:\n ##:::: ##:: ##:: ##. ####: ##. ##::: ##::::::: ##...:::: ##.... ##: ##...:::: ##.. ##::: ##::: ##::\n ##:::: ##:: ##:: ##:. ###: ##:. ##:: ##::::::: ##::::::: ##:::: ##: ##::::::: ##::. ##:: ##::: ##::\n ########::'####: ##::. ##: ##::. ##: ########: ########: ########:: ########: ##:::. ##:. ######:::\n........:::....::..::::..::..::::..::........::........::........:::........::..:::::..:::......::::".green());

	use colored::Colorize;
	use std::env;

	// get discord token
	let token = match env::var("DISCORD_TOKEN") {
		Ok(token) => token,
		Err(_) => {
			println!(
				"{}",
				"Please set the environment variable DISCORD_TOKEN to continue".red()
			);
			return;
		}
	};

	// print usage
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("{}", format!("Usage: {} <guid_id>", args[0]).red());
		return;
	}

	// parse guild
	let guild_id = match (&args[1]).parse::<u64>() {
		Ok(n) => n,
		Err(_) => panic!("{}", "Could not parse guid id".red()),
	};

	use handler::Handler;
	use serenity::{client::bridge::gateway::GatewayIntents, prelude::*};

	// create client with custom handler implementing trait `EventHandler`
	let mut client = Client::builder(token)
		.event_handler(Handler::new(guild_id))
		.intents(GatewayIntents::GUILD_PRESENCES | GatewayIntents::GUILD_MEMBERS)
		.await
		.unwrap_or_else(|_| panic!("{}", "Error creating client".red().to_string()));

	// start client or panic
	if let Err(why) = client.start().await {
		panic!("{}", format!("Client error: {:?}", why).red());
	}
}
