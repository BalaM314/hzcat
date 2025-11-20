use anyhow::{Context, Result, anyhow};
use std::{env::args, fs::File, io::{Read, Seek, SeekFrom, Write, stdout}};

fn main() -> Result<()> {
	let arg = args()
		.nth(1)
		.ok_or(anyhow!("Missing filename\nUsage: hzcat <file>"))?;
	match &arg[..] {
		"-?" | "--help" => {
			println!("Usage: hzcat <file>\nDecompresses a headerless file compressed with the DEFLATE algorithm.");
		},
		_ => {
			let file = File::open(&arg).context("Error reading file")?;
			let mut decompressed = String::new();
			match flate2::read::DeflateDecoder::new(file).read_to_string(&mut decompressed) {
				Ok(_) => print!("{}", decompressed),
				Err(e) => {
					let mut file = File::open(&arg).context("Error reading file")?;
					file.seek(SeekFrom::Start(10))?; //skip 10 bytes
					flate2::read::DeflateDecoder::new(file).read_to_string(&mut decompressed).map_err(|_| e)?;
				}
			}
			stdout().flush()?;
		}
	}
	Ok(())
}

