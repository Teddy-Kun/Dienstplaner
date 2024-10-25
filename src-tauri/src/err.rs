use serde::{Deserialize, Serialize};
use std::{fmt::Debug, process};

// Universal Error struct.
// Anything that can be converted to string can be converted to this Error.
#[derive(Serialize, Deserialize)]
pub struct GenericError {
	message: String,
}

impl GenericError {
	pub fn new(message: &str) -> GenericError {
		GenericError {
			message: message.to_string(),
		}
	}

	#[allow(dead_code)]
	pub fn out(&self) {
		eprintln!("{:?}", self);
	}

	#[allow(dead_code)]
	pub fn fatal(&self) {
		self.out();
		process::exit(1);
	}
}

impl<T: ToString> From<T> for GenericError {
	fn from(value: T) -> Self {
		GenericError::new(&value.to_string())
	}
}

impl Debug for GenericError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
