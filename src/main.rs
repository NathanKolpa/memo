use clap::{Arg, App};
use std::option::Option::Some;
use std::{env, io};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Write, Read};
use std::process::{Command, Stdio};

fn main() -> Result<(), std::io::Error> {
	let matches = App::new("memo")
		.version("1.0")
		.author("Nathan Kolpa <nathan@kolpa.me>")
		.about("Memoize program output")
		.arg(
			Arg::new("KEY")
				.value_name("KEY")
				.takes_value(true)
				.index(1)
				.about("The key for your memo")
				.required(true)
		)
		.arg(
			Arg::new("COMMAND")
				.value_name("COMMAND")
				.about("The command to execute")
				.takes_value(true)
				.multiple(true)
				.min_values(1)
				.required(true)
				.index(2)
		)
		.get_matches();

	let key = matches.value_of("KEY").unwrap();
	let dir = env::temp_dir().join("memo");

	if !dir.exists() {
		std::fs::create_dir(dir.clone())?;
	}

	let key_hash = Sha256::digest(key.as_bytes());
	let file_path = dir.join(format!("{:X}", key_hash));

	if file_path.exists() {
		let stdout = io::stdout();
		let mut stdout = stdout.lock();

		let mut file = File::open(file_path).expect("Unexpected file race condition");

		io::copy(&mut file, &mut stdout)?;
	} else {
		let cmd = matches.values_of("COMMAND").unwrap();
		let cmd: Vec<&str> = cmd.collect();
		let first = cmd.first().expect("No command provided");
		let args: Vec<&&str> = cmd.iter().skip(1).collect();

		let mut child = Command::new(first)
			.args(args)
			.stdout(Stdio::piped())
			.spawn()?;


		if let Some(ref mut stdout) = child.stdout {
			let mut file = File::create(file_path).expect("Could not create file");
			let mut out = std::io::stdout();

			let mut reader = BufReader::new(stdout);

			let mut buffer = [0; 2000];

			while let Ok(size) = reader.read(&mut buffer) {
				if size <= 0 {
					break;
				}

				let view = &buffer[0..size];

				out.write_all(view)?;
				file.write_all(view)?;
			}

			file.flush()?;
			out.flush()?;
		}
	}

	Ok(())
}
