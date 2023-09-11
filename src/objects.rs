use std::fmt;
use yaserde_derive::*;

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
prefix = "wus",
rename = "WalkupScanToCompDestination",
namespace = "wus: http://www.hp.com/schemas/imaging/con/ledm/walkupscan/2010/09/28",
namespace = "dd: http://www.hp.com/schemas/imaging/con/dictionaries/1.0/",
namespace = "dd3: http://www.hp.com/schemas/imaging/con/dictionaries/2009/04/06",
namespace = "scantype: http://www.hp.com/schemas/imaging/con/ledm/scantype/2008/03/17"
)]
pub struct WalkupDestination {
	#[yaserde(rename = "Hostname", prefix = "dd3")]
	pub hostname: String,
	#[yaserde(rename = "Name", prefix = "dd")]
	pub name: String,
	#[yaserde(rename = "LinkType", prefix = "wus")]
	pub link_type: String,
	#[yaserde(rename = "ResourceURI", prefix = "dd")]
	pub resource_uri: Option<String>,
	#[yaserde(rename = "WalkupScanToCompSettings", prefix = "wus")]
	pub settings: Option<WalkupScanToCompSettings>,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
prefix = "wus",
namespace = "wus: http://www.hp.com/schemas/imaging/con/ledm/walkupscan/2010/09/28",
rename = "WalkupScanToCompSettings",
)]
pub struct WalkupScanToCompSettings {
	#[yaserde(rename = "ScanSettings", prefix = "scantype")]
	pub settings: ChosenScanSettings,
	#[yaserde(rename = "Shortcut", prefix = "wus")]
	pub shortcut: String,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
prefix = "scantype",
namespace = "dd: http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
rename = "ScanSettings",
)]
pub struct ChosenScanSettings {
	#[yaserde(rename = "ScanPlexMode", prefix = "dd")]
	pub scan_plex_mode: String,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
prefix = "wus",
namespace = "wus: http://www.hp.com/schemas/imaging/con/ledm/walkupscan/2010/09/28",
namespace = "dd: http://www.hp.com/schemas/imaging/con/dictionaries/1.0/",
namespace = "dd3: http://www.hp.com/schemas/imaging/con/dictionaries/2009/04/06",
rename = "WalkupScanToCompDestinations"
)]
pub struct WalkupDestinations {
	#[yaserde(
	prefix = "wus",
	rename = "WalkupScanToCompDestination",
	)]
	pub destinations: Vec<WalkupDestination>
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19",
rename = "ScanStatus"
)]
pub struct ScanStatus {
	#[yaserde(rename = "ScannerStatus")]
	pub scanner_status: String,
	#[yaserde(rename = "AdfState")]
	pub adf_state: String,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19",
rename = "ScanSettings"
)]
pub struct ScanSettings {
	#[yaserde(rename = "XResolution")]
	pub x_resolution: i16,
	#[yaserde(rename = "YResolution")]
	pub y_resolution: i16,
	#[yaserde(rename = "XStart")]
	pub x_start: i32,
	#[yaserde(rename = "YStart")]
	pub y_start: i32,
	#[yaserde(rename = "Width")]
	pub width: i32,
	#[yaserde(rename = "Height")]
	pub height: i32,
	#[yaserde(rename = "Format")]
	pub format: String,
	#[yaserde(rename = "CompressionQFactor")]
	pub compression_q_factor: i32,
	#[yaserde(rename = "ColorSpace")]
	pub color_space: String,
	#[yaserde(rename = "BitDepth")]
	pub bit_depth: i8,
	#[yaserde(rename = "InputSource")]
	pub input_source: String,
	#[yaserde(rename = "GrayRendering")]
	pub gray_rendering: String,
	#[yaserde(rename = "ToneMap")]
	pub tone_map: ToneMap,
	#[yaserde(rename = "SharpeningLevel")]
	pub sharpening_level: i8,
	#[yaserde(rename = "NoiseRemoval")]
	pub noise_removal: i8,
	#[yaserde(rename = "ContentType")]
	pub content_type: String
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19",
rename = "ToneMap"
)]
pub struct ToneMap {
	#[yaserde(rename = "Gamma")]
	pub gamma: i32,
	#[yaserde(rename = "Brightness")]
	pub brightness: i32,
	#[yaserde(rename = "Contrast")]
	pub contrast: i32,
	#[yaserde(rename = "Highlite")]
	pub highlite: i32,
	#[yaserde(rename = "Shadow")]
	pub shadow: i32,
	#[yaserde(rename = "Threshold")]
	pub threshold: i32,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "wus: http://www.hp.com/schemas/imaging/con/ledm/walkupscan/2010/09/28",
prefix = "wus"
rename = "WalkupScanToCompEvent"
)]
pub struct WalkupScanToCompEvent {
	#[yaserde(rename = "EventType", prefix = "wus")]
	pub event_type: String,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "ev: http://www.hp.com/schemas/imaging/con/ledm/events/2007/09/16",
prefix = "ev"
rename = "EventTable"
)]
pub struct EventTable {
	#[yaserde(prefix = "ev", rename = "Event")]
	pub events: Vec<Event>,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "ev: http://www.hp.com/schemas/imaging/con/ledm/events/2007/09/16",
namespace = "dd: http://www.hp.com/schemas/imaging/con/dictionaries/1.0/",
prefix = "ev"
rename = "Event"
)]
pub struct Event {
	#[yaserde(prefix = "dd", rename = "AgingStamp")]
	pub aging_stamp: String,
	#[yaserde(prefix = "dd", rename = "UnqualifiedEventCategory")]
	pub unqualified_event_category: String,
	#[yaserde(rename = "Payload",)]
	pub payloads: Vec<Payload>,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "ev: http://www.hp.com/schemas/imaging/con/ledm/events/2007/09/16",
namespace = "dd: http://www.hp.com/schemas/imaging/con/dictionaries/1.0/",
prefix = "ev"
rename = "Payload"
)]
pub struct Payload {
	#[yaserde(prefix = "dd", rename = "ResourceURI")]
	pub resource_uri: String,
	#[yaserde(prefix = "dd", rename = "ResourceType")]
	pub resource_type: String,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "j: http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30",
prefix = "j"
rename = "Job"
)]
pub struct Job {
	#[yaserde(prefix = "j", rename = "JobUrl")]
	pub url: String,
	#[yaserde(prefix = "j", rename = "JobState")]
	pub state: String,
	#[yaserde(prefix = "j", rename = "JobSource")]
	pub source: String,
	#[yaserde(prefix = "j", rename = "JobCategory")]
	pub category: String,
	#[yaserde(rename = "ScanJob", namespace = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19")]
	pub scan_job: ScanJob,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19",
rename = "ScanJob"
)]
pub struct ScanJob {
	#[yaserde(rename = "PreScanPage")]
	pub pre_scan_page: Vec<PreScanPage>,
	#[yaserde(rename = "PostScanPage")]
	pub post_scan_page: Vec<PostScanPage>,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19",
rename = "PreScanPage"
)]
pub struct PreScanPage {
	#[yaserde(rename = "PageNumber")]
	pub number: i32,
	#[yaserde(rename = "PageState")]
	pub state: String,
	#[yaserde(rename = "BinaryURL")]
	pub binary_url: String,
	#[yaserde(rename = "ImageOrientation")]
	pub image_orientation: String,
}

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
namespace = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19",
rename = "PreScanPage"
)]
pub struct PostScanPage {
	#[yaserde(rename = "PageNumber")]
	pub number: i32,
	#[yaserde(rename = "PageState")]
	pub state: String,
	#[yaserde(rename = "TotalLines")]
	pub total_lines: i32,
}

#[derive(Debug, Clone)]
pub struct GetDestinationError;

#[derive(Debug, Clone)]
pub struct AddDestinationError;

#[derive(Debug, Clone)]
pub struct DeleteDestinationError;

#[derive(Debug, Clone)]
pub struct DownloadError;

#[derive(Debug, Clone)]
pub struct ApiError {
	pub details: String,
}

impl ApiError {
	pub fn new(msg: &str) -> ApiError {
		ApiError{details: msg.to_string()}
	}
}

impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}",self.details)
	}
}

impl fmt::Display for GetDestinationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error getting walkup destionations")
	}
}

impl fmt::Display for AddDestinationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error adding walkup destionation")
	}
}

impl fmt::Display for DeleteDestinationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error deleting walkup destionation")
	}
}

impl fmt::Display for DownloadError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error downloading file")
	}
}
