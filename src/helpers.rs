use crate::objects::{ScanSettings, ScanStatus, ToneMap, WalkupScanToCompSettings};

pub fn create_job(status: ScanStatus, settings: WalkupScanToCompSettings) -> ScanSettings {
	let source = match status.adf_state.as_str() {
		"Empty" => {"Platen"}
		"Loaded" => {"Adf"},
		_ => {panic!("Unexpected ADF State {}", status.adf_state)}
	};

	let (format, content) = match settings.shortcut.as_str() {
		"SaveDocument1" => { ("Jpeg", "Document") },
		"SavePhoto1" => { ("Jpeg", "Photo" ) },
		_ => {panic!("Unexpected shortcut {}", settings.shortcut)}
	};

	log::info!("Using configuration source: {}; content: {}; format: {}", source, content, format);

	ScanSettings {
		x_resolution: 200,
		y_resolution: 200,
		x_start: 33,
		y_start: 0,
		width: 2481,
		height: 3507,
		format: format.to_string(),
		compression_q_factor: 0,
		color_space: "Color".to_string(),
		bit_depth: 8,
		input_source: source.to_string(),
		gray_rendering: "NTSC".to_string(),
		tone_map: ToneMap {
			gamma: 1000,
			brightness: 1000,
			contrast: 1000,
			highlite: 179,
			shadow: 25,
			threshold: 0,
		},
		sharpening_level: 128,
		noise_removal: 0,
		content_type: content.to_string(),
	}
}
