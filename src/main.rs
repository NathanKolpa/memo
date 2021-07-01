use clap::{Arg, App};
use std::option::Option::Some;
use std::{env, io};
use sha2::{Sha256, Digest};
use sha2::digest::DynDigest;
use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom, Write, LineWriter};
use std::process::{Command, Stdio};

fn main() {
	let matches = App::new("memo")
		.version("1.0")
		.author("Nathan Kolpa <nathan@kolpa.me>")
		.about("Memoize program output")
		.arg(
			Arg::new("KEY")
				.value_name("KEY")
				.takes_value(true)
				.short('k')
				.long("key")
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
		)
		.get_matches();

	if let Some(key) = matches.value_of("KEY") {
		let mut dir = env::temp_dir().join("memo");
		std::fs::create_dir(dir.clone());

		let key_hash = Sha256::digest(key.as_bytes());
		let file_path = dir.join(format!("{:X}", key_hash));

		if file_path.exists() {
			let stdout = io::stdout();
			let mut stdout = stdout.lock();

			let mut file = File::open(file_path).expect("Unexpected file race condition");

			io::copy(&mut file, &mut stdout);
		} else {
			if let Some(cmd) = matches.values_of("COMMAND") {
				let cmd: Vec<&str> = cmd.collect();
				let first = cmd.first().expect("No command provided");
				let args: Vec<&&str> = cmd.iter().skip(1).collect();

				let mut child = Command::new(first)
					.args(args)
					.stdout(Stdio::piped())
					.spawn()
					.expect("Failed to execute command");


				if let Some(ref mut stdout) = child.stdout {
					let mut file = File::create(file_path).expect("Could not create file");
					let mut file = LineWriter::new(file);

					let mut reader = BufReader::new(stdout);

					let mut line = String::new();
					while let Ok(size) = reader.read_line(&mut line) {
						if size <= 0 {
							break;
						}

						println!("{}", line);
						file.write_all(line.as_ref());

						line.clear();
					}

					file.flush();
				}

			}
		}
	}
}
