#[cfg(target_os = "linux")]
use ashpd::desktop::Color;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorSchemeAccent {
	hex_code: String,
}

impl ColorSchemeAccent {
	pub fn new(red: u8, green: u8, blue: u8) -> ColorSchemeAccent {
		ColorSchemeAccent {
			hex_code: format!("#{:02x}{:02x}{:02x}", red, green, blue),
		}
	}
}

#[cfg(target_os = "linux")]
impl From<Color> for ColorSchemeAccent {
	fn from(value: Color) -> Self {
		ColorSchemeAccent::new(
			(255.0 * value.red()) as u8,
			(255.0 * value.green()) as u8,
			(255.0 * value.blue()) as u8,
		)
	}
}

#[cfg(target_os = "windows")]
pub fn get_accent_color_windows() -> Result<ColorSchemeAccent, crate::err::GenericError> {
	let mut colorization: u32 = 0;
	let mut opaqueblend = windows::Win32::Foundation::BOOL(0);
	unsafe {
		windows::Win32::Graphics::Dwm::DwmGetColorizationColor(
			&mut colorization,
			&mut opaqueblend,
		)?;
	}

	let argb = hex::decode(format!("{:X}", colorization)).unwrap();
	println!("argb {:?}", argb);

	Ok(ColorSchemeAccent::new(argb[1], argb[2], argb[3]))
}
