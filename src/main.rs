use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use reqwest::Url;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use uuid::Uuid;
use crate::helpers::create_job;
use crate::hp_api::HpApi;
use crate::objects::{Event, Payload, WalkupDestination};

mod objects;
mod hp_api;
mod helpers;

fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

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
	let _ = api.get_eventtable();
	drop(api);

	loop {
		log::info!("Waiting for job!");
		let mut api = lock2.lock().unwrap();
		let event_table = api.get_eventtable_timeout(1200).unwrap();
		let target_event = "ScanEvent".to_string();
		let events = event_table.events.iter()
			.filter(|event| *event.unqualified_event_category == target_event)
			.collect::<Vec<&Event>>();
		if !events.is_empty() {
			for event in events {
				let target_resource = "wus:WalkupScanToCompDestination".to_string();
				let payloads = event.payloads.iter()
					.filter(|payload| *payload.resource_type == target_resource)
					.collect::<Vec<&Payload>>();
				for destination in api.active_destinations.clone() {
					if payloads.first()
						.unwrap()
						.resource_uri.contains(destination.to_string().as_str()) {
						start_scanning(&api, destination);
					}
				}
			}
		}
	}


	// ENDE
	Ok(())
}

fn start_scanning(api: &HpApi, target_destination: Uuid) {
	let event = api.get_scantocomp_event().unwrap();
	if event.event_type != "ScanRequested" {
		log::warn!("Unexpected ScanType while scanning {}", event.event_type);
		return
	}
	let destination = api
		.get_walkup_destionation(target_destination);

	let expected = destination
		.expect("Error getting scan destination");

	let settings = expected.settings.to_owned()
		.expect("Settings did not contain no pressed shortcut");

	let scan_status = api.get_scanner_status()
		.expect("Error getting scanner status");

	let job = create_job(scan_status, settings);
	let job_location = api.create_job(job)
		.expect("Error posting job");

	loop {
		log::info!("Still waiting for scanner");
		let job_info = api.get_job_with_url(&job_location)
			.expect("Error getting posted job information");

		if job_info.scan_job.pre_scan_page.is_empty() { return }

		let first_page = job_info.scan_job.pre_scan_page.first().expect("No pre-scan-pages");

		if first_page.state == "PreparingScan" {
			// sleeping a bit to not hammer the printer
			thread::sleep(time::Duration::from_millis(300));
			continue
		}

		if first_page.state == "ReadyToUpload" {
			let _ = api.download_page(&first_page.binary_url);
		}
	}
}

fn shutdown(api: &Arc<Mutex<HpApi>>) {
	let mut api = api.lock().unwrap();
	api.cleanup();
	std::process::exit(0);
}
