use std::{
	fs::File,
	io::{Error, Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
	pub use_widget: bool,
	pub include_only: Vec<u64>,
}

impl Settings {
	pub fn new() -> Settings {
		Settings {
			use_widget: true,
			include_only: vec![],
		}
	}

	fn save(&self) -> Result<(), Error> {
		let mut file = File::create("settings.json")?;
		let json = serde_json::to_string(&self)?;
		file.write(json.as_bytes())?;
		Ok(())
	}

	pub fn load() -> Result<Settings, Error> {
		let mut file = match File::open("settings.json") {
			Ok(file) => file,
			Err(_) => {
				let settings = Settings::new();
				settings.save()?;
				return Ok(settings);
			}
		};

		let mut buf = String::new();
		file.read_to_string(&mut buf)?;
		Ok(serde_json::from_str(&buf)?)
	}
}
