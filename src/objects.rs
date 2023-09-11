use std::fmt;
use yaserde_derive::*;

#[derive(YaDeserialize, YaSerialize, Default, Debug, PartialEq, Clone)]
#[yaserde(
prefix = "wus",
rename = "WalkupScanToCompDestination",
namespace = "wus: http://www.hp.com/schemas/imaging/con/ledm/walkupscan/2010/09/28",
namespace = "dd: http://www.hp.com/schemas/imaging/con/dictionaries/1.0/",
namespace = "dd3: http://www.hp.com/schemas/imaging/con/dictionaries/2009/04/06",
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

#[derive(Debug, Clone)]
pub struct GetDestinationError;

#[derive(Debug, Clone)]
pub struct AddDestinationError;

impl fmt::Display for GetDestinationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error getting walkup destionations")
	}
}

impl fmt::Display for AddDestinationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error adding walkup destionations")
	}
}
