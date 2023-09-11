use reqwest::blocking::{ClientBuilder, Client};
use reqwest::{StatusCode, Url};
use uuid::Uuid;
use yaserde::de::from_str;
use yaserde::ser::to_string;
use crate::objects::{AddDestinationError, GetDestinationError, WalkupDestination, WalkupDestinations};

pub struct HpApi {
	client: Client,
	base_url: Url
}

impl<'a> HpApi {
	pub fn new(base_url: Url) -> HpApi {
		let client = ClientBuilder::new()
			.http1_title_case_headers()
			.build()
			.expect("Error building the HP API Client");

		log::debug!("Base URL: {}", base_url);

		HpApi {
			client,
			base_url,
		}
	}

	pub fn get_walkup_destinations(&'a self) -> Result<WalkupDestinations, GetDestinationError> {
		log::debug!("Making request for WalkupScanToCompDestinations");

		let url = self.base_url.join("WalkupScanToComp/WalkupScanToCompDestinations")
			.expect("Error generating URL");

		let resp = self.client.get(url)
			.send()
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

	pub fn add_destination(&'a self, new_destination: WalkupDestination) -> Result<Uuid, AddDestinationError> {
		let str = to_string(&new_destination)
			.expect("Error converting WalkupDestionation to XML");

		let client = reqwest::blocking::ClientBuilder::new()
			.http1_title_case_headers()
			.build()
			.expect("Error creating reqwest Cleint");

		let url = self.base_url.join("/WalkupScanToComp/WalkupScanToCompDestinations")
			.expect("Error generating URL");

		log::debug!("New URL: {}", url);

		let response = client.post(url)
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
}
