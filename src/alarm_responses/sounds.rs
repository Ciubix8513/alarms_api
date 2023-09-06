#![allow(clippy::module_name_repetitions)]

use std::{fs::File, io::BufReader};

use rodio::{Decoder, Source};

use crate::config::FileArguments;

pub fn alarm(args: &FileArguments) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, handle) = rodio::OutputStream::try_default()?;
    let file = File::open(&args.file_path)?;
    let file = BufReader::new(file);
    let decoder = Decoder::new(file)?;

    handle.play_raw(decoder.convert_samples())?;
    Ok(())
}
