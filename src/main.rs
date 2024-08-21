mod audio_controller;
mod audio_ses_definitions;
mod com_guard;
mod sink;

use crate::sink::Sink;
use audio_controller::AudioController;
use rodio::{Decoder, OutputStream};
use std::io::{Cursor, Error};
use std::{env, fmt, fs};

fn main() -> Result<(), ApplicationError> {
	let args: Vec<String> = env::args().collect();

	let ac = AudioController::new()?;
	if args.len() == 2 && args[1].eq("-l") {
		let listing = ac.list_audio_sinks()?;
		for s in listing {
			println!("{:?}", s);
		}
		return Ok(());
	}

	let (dev1, dev2) = match process_names_config() {
		Ok(el) => { el }
		Err(e) => {
			print_help();

			return Err(ApplicationError::Custom("Invalid arguments. Use -l to list audio sinks or specify sinks to switch.".to_string()));
		}
	};

	process_switching(&ac, &dev1, &dev2)?;
	play_beep()?;
	Ok(())
}

fn play_beep() -> Result<(), ApplicationError> {
	let beep_wav = include_bytes!("./assets/beeping.wav");
	let decoder = Decoder::new_wav(Cursor::new(beep_wav))?;

	let (_stream, handle) = OutputStream::try_default()?;
	let sink = rodio::Sink::try_new(&handle)?;
	sink.append(decoder);
	sink.sleep_until_end();

	Ok(())
}

fn process_names_config() -> Result<(String, String), Error> {
	let args: Vec<String> = env::args().collect();

	if args.len() == 3 {
		let searched_str_1 = &args[1];
		let searched_str_2 = &args[2];
		return Ok((searched_str_1.to_owned(), searched_str_2.to_owned()));
	}

	let s = match fs::read_to_string("./config") {
		Ok(el) => {
			el
		}
		Err(ee) => { //TODO process error
			let mut b = env::current_exe()?.parent().unwrap().to_owned();
			b.push("config");
			fs::read_to_string(b)?
		}
	};

	let mut split = s.split("\n");

	Ok((
		split.next().unwrap().trim().to_owned(),
		split.next().unwrap().trim().to_owned()
	))
}

fn process_switching(ac: &AudioController, name1: &str, name2: &str) -> Result<(), ApplicationError> {
	let default_sink = ac.get_default_endpoint()?;
	let default_id = default_sink.id();

	let (device1, device2) = find_devices(ac, name1, name2)?;
	let chosen: &Sink = if default_id.eq(device1.id()) {
		&device2
	} else {
		&device1
	};

	println!("Setting default to {}", chosen.name());
	ac.set_default_audio_sink(chosen.id())?;

	Ok(())
}

fn find_devices(ac: &AudioController, searched_str_1: &str, searched_str_2: &str) -> Result<(Sink, Sink), ApplicationError> {
	let listing = ac.list_audio_sinks()?;

	let mut first_device: Sink = Sink::default();
	let mut second_device: Sink = Sink::default();;

	let lowercase_str1 = searched_str_1.to_lowercase();
	let lowercase_str2 = searched_str_2.to_lowercase();
	for s in listing {
		let lower_name = s.name().to_lowercase();
		if lower_name.contains(&lowercase_str1) {
			first_device = s;
		} else if lower_name.contains(&lowercase_str2) {
			second_device = s;
		}
	}

	Ok((first_device, second_device))
}

#[derive(Debug)]
enum ApplicationError {
	WrongArgumentsError,
	Custom(String),
	ComError(String),
	SoundError(String),
}

impl fmt::Display for ApplicationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Permission denied")
	}
}

impl From<windows::core::Error> for ApplicationError {
	fn from(e: windows::core::Error) -> Self {
		ApplicationError::ComError(e.to_string())
	}
}

impl From<hound::Error> for ApplicationError {
	fn from(e: hound::Error) -> Self {
		ApplicationError::SoundError(e.to_string())
	}
}

impl From<rodio::StreamError> for ApplicationError {
	fn from(e: rodio::StreamError) -> Self {
		ApplicationError::SoundError(e.to_string())
	}
}

impl From<rodio::PlayError> for ApplicationError {
	fn from(e: rodio::PlayError) -> Self {
		ApplicationError::SoundError(e.to_string())
	}
}

impl From<rodio::decoder::DecoderError> for ApplicationError {
	fn from(e: rodio::decoder::DecoderError) -> Self {
		ApplicationError::SoundError(e.to_string())
	}
}

fn print_help() {
	println!("Usage:");
	println!("\t-l List all audio sinks");
	println!("\t[SYNK1] [SINK2] switch current default between the mentioned sinks");
	println!("Or create a 'config' fie with 2 lines indicate which sink in each one of them");
}