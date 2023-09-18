use reqwest::blocking::{ClientBuilder, Client};
use reqwest::{StatusCode, Url};
use uuid::Uuid;
use yaserde::de::from_str;
use yaserde::ser::to_string;
use crate::objects::{AddDestinationError, DeleteDestinationError, GetDestinationError, DownloadError, WalkupDestination, WalkupDestinations, WalkupScanToCompEvent, ApiError, EventTable, Job, ScanSettings};

pub struct HpApi {
	client: Client,
	base_url: Url,
	active_destinations: Vec<Uuid>
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
			active_destinations: Vec::new()
		}
	}

	pub fn cleanup(&'a mut self) {
		for destination in self.active_destinations.clone() {
			let _ = self.delete_destination(destination);
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

	pub fn add_destination(&'a mut self, new_destination: WalkupDestination) -> Result<Uuid, AddDestinationError> {
		let str = to_string(&new_destination)
			.expect("Error converting WalkupDestionation to XML");

		let url = self.base_url.join("/WalkupScanToComp/WalkupScanToCompDestinations")
			.expect("Error generating URL");

		let response = self.client.post(url)
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

				self.active_destinations.push(uuid);

				log::debug!("Destination UUID: {}", &uuid);

				Ok(uuid)
			}
			_ => {
				log::error!("Error adding a destination! Response code: {}", response.status());
				Err(AddDestinationError)
			}
		}
	}

	pub fn delete_destination(&'a mut self, uuid: Uuid) -> Result<(), DeleteDestinationError> {
		log::info!("Deleteing destination with uuid {}", &uuid);

		let path = format!("/WalkupScanToComp/WalkupScanToCompDestinations/{}", &uuid);

		let url = self.base_url.join(&path)
			.expect("Error generating URL");

		let response = self.client.delete(url)
			.send()
			.expect("Error sending DELETE WalkupScanToCompDestinations request");

		match response.status() {
			StatusCode::OK => {
				self.active_destinations.iter()
					.position(|id| *id == uuid)
					.expect("Could not find uuid in active destinations");
				log::info!("Deletion successful");
				Ok(())
			},
			_ => {
				log::error!("Error deleting destination");
				Err(DeleteDestinationError)
			}
		}
	}

	pub fn get_eventtable(&'a self) -> Result<EventTable, ApiError> {
		let url = self.base_url.join("/EventMgmt/EventTable")
			.expect("Error generating URL");

		let response = self.client.get(url)
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				let text = response
					.text()
					.expect("Error reading text from response");
				Ok(from_str(&text).unwrap())
			}
			_ => {
				Err(ApiError::new("Error reading EventTable"))
			}
		}
	}

	pub fn get_eventtable_timeout(&'a self, timeout: i32) -> Result<EventTable, ApiError> {
		let url = self.base_url.join("/EventMgmt/EventTable")
			.expect("Error generating URL");

		let query = vec![("timeout", timeout)];

		let response = self.client.get(url)
			.query(&query)
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				let text = response
					.text()
					.expect("Error reading text from response");
				Ok(from_str(&text).unwrap())
			}
			_ => {
				Err(ApiError::new("Error reading EventTable"))
			}
		}
	}

	pub fn create_job(&'a self, job: ScanSettings) -> Result<String, ApiError> {
		let str = to_string(&job)
			.expect("Error converting ScanSettings to XML");

		let url = self.base_url.join("/Scan/Jobs")
			.expect("Error generating URL");

		let response = self.client.post(url)
			.header("Content-Type", "text/xml")
			.body(str)
			.send()
			.expect("Error sending POST Scan/Jobs request");

		match response.status() {
			StatusCode::CREATED => {
				let location = response
					.headers()
					.get("Location")
					.expect("Missing Location header in response")
					.to_str()
					.expect("Could not map Location header to string");

				log::info!("Successfully created new Scan Job with url {}", location);
				Ok(location.to_string())
			}
			_ => {
				log::error!("Error adding a scan job! Response code: {}", response.status());
				Err(ApiError::new("Error adding a scan job"))
			}
		}
	}

	pub fn get_job_with_url(&'a self, url: String) -> Result<Job, ApiError> {
		let response = self.client.get(url)
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				let text = response
					.text()
					.expect("Error reading text from response");
				Ok(from_str(&text).unwrap())
			}
			_ => {
				Err(ApiError::new("Error reading WalkupScanToCompEvent"))
			}
		}
	}

	pub fn get_scantocomp_event(&'a self) -> Result<WalkupScanToCompEvent, ApiError> {
		let url = self.base_url.join("/WalkupScanToComp/WalkupScanToCompEvent")
			.expect("Error generating URL");

		let response = self.client.get(url)
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				let text = response
					.text()
					.expect("Error reading text from response");
				Ok(from_str(&text).unwrap())
			}
			_ => {
				Err(ApiError::new("Error reading WalkupScanToCompEvent"))
			}
		}
	}

	pub fn download_page(&'a self, path: String) -> Result<(), DownloadError> {
		let url = self.base_url.join(&path)
			.expect("Error generating URL");
		let response = self.client.get(url)
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				log::info!("Download Successful");
				Ok(())
			},
			_ => {
				log::error!("Error downloading page");
				Err(DownloadError)
			}
		}
	}
}
