use std::error::Error;
use reqwest::StatusCode;
use yaserde::de::from_str;
use yaserde::ser::to_string;
use crate::objects::{AddDestinationError, GetDestinationError, WalkupDestination, WalkupDestinations};
use uuid::Uuid;

mod objects;

fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

	let dest = WalkupDestination {
		hostname: "Rust".to_string(),
		name: "Rust".to_string(),
		link_type: "Network".to_string(),
		resource_uri: None,
	};

	let _ = add_destination(dest);
	Ok(())
}

fn add_destination(new_destination: WalkupDestination) -> Result<Uuid, AddDestinationError> {
	let str = to_string(&new_destination)
		.expect("Error converting WalkupDestionation to XML");

	let client = reqwest::blocking::ClientBuilder::new()
		.http1_title_case_headers()
		.build()
		.expect("Error creating reqwest Cleint");

	let response = client.post("http://192.168.2.3/WalkupScanToComp/WalkupScanToCompDestinations")
		.header("Content-Type", "text/xml")
		.body(str)
		.send()
		.expect("Error sending POST WalkupScanToCompDestinations request");

	match response.status() {
		StatusCode::CREATED => {
			let location = response
				.headers()
				.get("Location")
				.expect("Missing Location header in response")
				.to_str()
				.expect("Could not map Location header to string");

			log::info!("Successfully created new WalkupScanToCompDestinations with name {}", new_destination.name);
			log::debug!("Using location URL: {} to generate UUID", location);

			let url_parts = location.split("/")
				.collect::<Vec<&str>>();

			let uuid_string = url_parts.last()
				.expect("Could not get last part of Location URL");

			let uuid = Uuid::parse_str(uuid_string)
				.expect("Location URL did not contain a valid UUID");

			log::debug!("Destination UUID: {}", &uuid);

			return Ok(uuid)
		}
		_ => {
			log::error!("Error adding a destination! Response code: {}", response.status());
			return Err(AddDestinationError)
		}
	}
}

fn get_walkup_destinations() -> Result<WalkupDestinations, GetDestinationError> {
	log::debug!("Making request for WalkupScanToCompDestinations");
	let resp = reqwest::blocking::get("http://192.168.2.3/WalkupScanToComp/WalkupScanToCompDestinations")
		.expect("Error making GET WalkupScanToCompDestinations request")
		.text()
		.expect("Answer to GET WalkupScanToCompDestinations request did not contain text");
	let deser: Result<WalkupDestinations, String> = from_str(&resp);

	match deser {
		Ok(dests) => {
			log::debug!("Got list of walkup destinations with {} destinations", dests.destinations.iter().count());
			Ok(dests)
		}
		Err(_) => {
			Err(GetDestinationError)
		}
	}
}
