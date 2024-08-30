use std::error::Error;
use std::fmt;
use rodio::decoder::DecoderError;
use rodio::{PlayError, StreamError};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ApplicationError {
	ConfigError,
	WinError(windows::core::Error),
	GeneralError(Box<dyn Error>),
}

impl fmt::Display for ApplicationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl Error for ApplicationError {}

impl From<windows::core::Error> for ApplicationError {
	fn from(e: windows::core::Error) -> Self {
		ApplicationError::WinError(e)
	}
}

macro_rules! implement_err_conversion {
    ($T: ty) => {
impl From<$T> for ApplicationError {
	fn from(e: $T) -> Self {
		ApplicationError::GeneralError(Box::new(e))
	}
}
    };
}

implement_err_conversion!(DecoderError);
implement_err_conversion!(StreamError);
implement_err_conversion!(PlayError);

#[cfg(test)]
mod tests {
	use rodio::StreamError;
	use windows::core::HRESULT;
	use crate::application_errors::ApplicationError;

	#[test]
	fn application_error_message() {
		let error = ApplicationError::ConfigError;
		let ss = error.to_string();
		assert_eq!("ConfigError", ss);
	}

	#[test]
	fn windows_error_message() {
		let hresult = HRESULT::from_nt(10);
		let win_err = windows::core::Error::from(hresult);
		let app_err = ApplicationError::WinError(win_err);

		assert_eq!(r#"WinError(Error { code: HRESULT(0x0000000A), message: "The environment is incorrect." })"#, app_err.to_string())
	}

	#[test]
	fn sound_error_message() {
		let no_device = StreamError::NoDevice;
		assert_eq!("GeneralError(NoDevice)", ApplicationError::from(no_device).to_string());
	}
}