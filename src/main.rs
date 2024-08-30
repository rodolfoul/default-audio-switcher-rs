mod audio_controller;
mod audio_ses_definitions;
mod com_guard;
mod sink;
mod application_errors;

use crate::application_errors::ApplicationError;
use crate::sink::Sink;
use audio_controller::AudioController;
use rodio::{Decoder, OutputStream};
use std::io::{Cursor, Error, ErrorKind};
use std::{env, fs};

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
		Ok((a, b)) => { (a, b) }
		Err(e) => {
			print_help();
			return Err(e);
		}
	};

	process_sink_switching(&ac, &dev1, &dev2)?;
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

fn process_names_config() -> Result<(String, String), ApplicationError> {
	let args: Vec<String> = env::args().collect();

	if args.len() == 3 {
		let searched_str_1 = &args[1];
		let searched_str_2 = &args[2];
		return Ok((searched_str_1.to_owned(), searched_str_2.to_owned()));
	}

	let s = match read_config_file() {
		Ok(s) => { s }
		Err(e) => {
			return Err(match e.kind() {
				ErrorKind::NotFound => {
					ApplicationError::ConfigError
				}
				_ => { ApplicationError::GeneralError(Box::new(e)) }
			})
		}
	};

	let mut split = s.split("\n");

	Ok((
		split.next().unwrap().trim().to_owned(),
		split.next().unwrap().trim().to_owned()
	))
}

fn read_config_file() -> Result<String, Error> {
	//Try current directory
	let err = match fs::read_to_string("config") {
		Ok(el) => { return Ok(el) }
		Err(ee) => { ee }
	};

	//Try executable directory
	let exe_search_result = match err.kind() {
		ErrorKind::NotFound => {
			let mut b = env::current_exe()?.parent().unwrap().to_owned();
			b.push("config");
			fs::read_to_string(b)
		}
		_ => {
			return Err(err)
		}
	};

	exe_search_result
}

fn process_sink_switching(ac: &AudioController, name1: &str, name2: &str) -> Result<(), ApplicationError> {
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
	let mut second_device: Sink = Sink::default();

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

fn print_help() {
	println!("Usage:");
	println!("\t-l List all audio sinks");
	println!("\t[SYNK1] [SINK2] switch current default between the mentioned sinks");
	println!("Or create a 'config' file with 2 lines. Indicate which sink in each one of them.");
	println!();
}

#[cfg(test)]
mod tests {
	use crate::application_errors::ApplicationError;
	use crate::main;
	use std::env::{current_dir, set_current_dir};

	#[test]
	fn no_config_error() {
		let saved_dir = current_dir().unwrap();
		set_current_dir("..").unwrap();
		let err = main().unwrap_err();
		set_current_dir(saved_dir).unwrap();

		match err {
			ApplicationError::ConfigError => {}
			_ => panic!("Not expected error!")
		}
	}
}