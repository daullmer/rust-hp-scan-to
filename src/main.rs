use std::error::Error;
use reqwest::Url;
use crate::hp_api::HpApi;
use crate::objects::WalkupDestination;

mod objects;
mod hp_api;

fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

	let base_url = Url::parse("http://192.168.2.3")?;
	let api = HpApi::new(base_url);

	let dest = WalkupDestination {
		hostname: "Rust".to_string(),
		name: "Rust".to_string(),
		link_type: "Network".to_string(),
		resource_uri: None,
		settings: None,
	};

	let uuid = api.add_destination(dest).unwrap();
	let _ = api.get_walkup_destinations();
	let _ = api.delete_destination(uuid);
	Ok(())
}
