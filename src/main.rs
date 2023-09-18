use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use reqwest::Url;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use crate::hp_api::HpApi;
use crate::objects::WalkupDestination;

mod objects;
mod hp_api;

fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

	let base_url = Url::parse("http://192.168.2.3")?;
	let api = HpApi::new(base_url);
	let arc_api = Arc::new(Mutex::new(api));
	let lock2 = Arc::clone(&arc_api);

	let mut signals = Signals::new(TERM_SIGNALS)?;

	thread::spawn(move || {
		log::debug!("Spawned new thread monitoring signals");
		for signal in signals.forever() {
			log::info!("Received shutdown signal {:?}", signal);
			shutdown(&arc_api);
		}
	});

	let dest = WalkupDestination {
		hostname: "Rust".to_string(),
		name: "Rust".to_string(),
		link_type: "Network".to_string(),
		resource_uri: None,
		settings: None,
	};

	let mut api = lock2.lock().unwrap();
	let _ = api.add_destination(dest).unwrap();
	let _ = api.get_walkup_destinations();


	// ENDE
	api.cleanup();
	drop(api);
	Ok(())
}

fn shutdown(api: &Arc<Mutex<HpApi>>) {
	let mut api = api.lock().unwrap();
	api.cleanup();
	std::process::exit(0);
}
