use std::fs::File;
use std::io::{copy, Cursor};
use std::time::Duration;
use reqwest::blocking::{ClientBuilder, Client};
use reqwest::{StatusCode, Url};
use uuid::Uuid;
use yaserde::de::from_str;
use yaserde::ser::to_string;
use crate::objects::{AddDestinationError, DeleteDestinationError, GetDestinationError, DownloadError, WalkupDestination, WalkupDestinations, WalkupScanToCompEvent, ApiError, EventTable, Job, ScanSettings, ScanStatus};

pub struct HpApi {
	client: Client,
	base_url: Url,
	pub active_destinations: Vec<Uuid>,
	last_known_etag: Option<String>
}

impl<'a> HpApi {
	pub fn new(base_url: Url) -> HpApi {
		let client = ClientBuilder::new()
			.http1_title_case_headers()
			.timeout(Duration::from_secs(3 * 60))
			.build()
			.expect("Error building the HP API Client");

		log::debug!("Base URL: {}", base_url);

		HpApi {
			client,
			base_url,
			active_destinations: Vec::new(),
			last_known_etag: None
		}
	}

	pub fn cleanup(&'a mut self) {
		for destination in self.active_destinations.clone() {
			let _ = self.delete_destination(destination);
		}
	}

	pub fn connection_check(&'a self) -> bool {
		log::debug!("Checking printer availability");
		let client = ClientBuilder::new()
			.http1_title_case_headers()
			.timeout(Duration::from_secs(5))
			.build()
			.expect("Error building the HP API Client");

		let url = self.base_url.join("Scan/Status")
			.expect("Error generating URL");

		match client.get(url).send() {
			Ok(_) => {
				log::debug!("Printer reachable!");
				true
			}
			Err(_) => {
				log::debug!("Printer not reachable!");
				false
			}
		}
	}

	#[allow(dead_code)]
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

	pub fn get_walkup_destionation(&'a self, uuid: Uuid) -> Result<WalkupDestination, GetDestinationError> {
		log::debug!("Making request for WalkupScanToCompDestinations {}", uuid);

		let url = self.base_url.join("WalkupScanToComp/WalkupScanToCompDestinations/")
			.expect("Error generating URL")
			.join(uuid.to_string().as_str())
			.expect("Error generating URL");

		let resp = self.client.get(url)
			.send()
			.expect("Error making GET WalkupScanToCompDestinations request")
			.text()
			.expect("Answer to GET WalkupScanToCompDestinations request did not contain text");
		let deser: Result<WalkupDestination, String> = from_str(&resp);

		match deser {
			Ok(dests) => {
				log::debug!("Got walkup destinations");
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

				log::info!("Destination UUID: {}", &uuid);

				Ok(uuid)
			}
			_ => {
				log::error!("Error adding a destination! Response code: {}", response.status());
				Err(AddDestinationError)
			}
		}
	}

	pub fn delete_destination(&'a mut self, uuid: Uuid) -> Result<(), DeleteDestinationError> {
		log::debug!("Deleteing destination with uuid {}", &uuid);

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
				log::info!("Deletion of destination {} successful", uuid);
				Ok(())
			},
			_ => {
				log::error!("Error deleting destination");
				Err(DeleteDestinationError)
			}
		}
	}

	pub fn get_eventtable(&'a mut self) -> Result<EventTable, ApiError> {
		log::debug!("Getting eventtable");

		let url = self.base_url.join("/EventMgmt/EventTable")
			.expect("Error generating URL");

		let mut request = self.client.get(url);

		if let Some(etag) = self.last_known_etag.clone() {
			request = request.header("If-None-Match", etag);
		}

		let response = request
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				let text = response
					.text()
					.expect("Error reading text from response");

				let table: EventTable = from_str(&text).unwrap();
				if let Some(event) = table.events.last() {
					log::debug!("Setting last known etag to {}", event.aging_stamp);
					self.last_known_etag = Some(event.aging_stamp.clone())
				};
				Ok(table)
			}
			_ => {
				Err(ApiError::new("Error reading EventTable"))
			}
		}
	}

	pub fn get_eventtable_timeout(&'a mut self, timeout: i32) -> Result<EventTable, ApiError> {
		log::debug!("Getting eventtable with timeout {}", timeout);

		let url = self.base_url.join("/EventMgmt/EventTable")
			.expect("Error generating URL");

		let query = vec![("timeout", timeout)];

		let mut request = self.client.get(url)
			.query(&query);

		if let Some(etag) = self.last_known_etag.clone() {
			request = request.header("If-None-Match", etag);
		}

		let response = request
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				let text = response
					.text()
					.expect("Error reading text from response");

				let table: EventTable = from_str(&text).unwrap();
				if let Some(event) = table.events.last() {
					log::debug!("Setting last known etag to {}", event.aging_stamp);
					self.last_known_etag = Some(event.aging_stamp.clone())
				};
				Ok(table)
			}
			_ => {
				Err(ApiError::new("Error reading EventTable"))
			}
		}
	}

	pub fn create_job(&'a self, job: ScanSettings) -> Result<String, ApiError> {
		log::debug!("Creating new scan job");

		let str = to_string(&job)
			.expect("Error converting ScanSettings to XML");

		let url = self.base_url.join("/Scan/Jobs")
			.expect("Error generating URL");

		log::debug!("Post body: {}", str);

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

				log::debug!("Successfully created new Scan Job with url {}", location);
				Ok(location.to_string())
			}
			_ => {
				log::error!("Error adding a scan job! Response code: {}", response.status());
				Err(ApiError::new("Error adding a scan job"))
			}
		}
	}

	pub fn get_job_with_url(&'a self, url: &String) -> Result<Job, ApiError> {
		log::debug!("Getting job with url");

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

	pub fn get_scanner_status(&'a self) -> Result<ScanStatus, ApiError> {
		let url = self.base_url.join("/Scan/Status")
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

	pub fn download_page(&'a self, path: &String) -> Result<File, DownloadError> {
		let url = self.base_url.join(&path)
			.expect("Error generating URL");
		let response = self.client.get(url)
			.send()
			.expect("Error sending download page request");

		match response.status() {
			StatusCode::OK => {
				let mut file = File::create("./file.pdf")
					.expect("could not creat file");

				let mut content =  Cursor::new(response.bytes()
					.expect("Could not turn response into bytes"));
				let _ = copy(&mut content, &mut file);
				log::debug!("Download Successful");
				Ok(file)
			},
			_ => {
				log::error!("Error downloading page");
				Err(DownloadError)
			}
		}
	}
}
