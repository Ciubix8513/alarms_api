#![allow(clippy::module_name_repetitions)]

use std::{fs::File, io::BufReader};

use rodio::{Decoder, Sink};

use crate::config::FileArguments;

pub fn alarm(args: &FileArguments) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, handle) = rodio::OutputStream::try_default()?;
    let file = File::open(&args.file_path)?;
    let file = BufReader::new(file);
    let decoder = Decoder::new(file)?;

    let sink = Sink::try_new(&handle)?;
    sink.append(decoder);
    // handle.play_raw(decoder.convert_samples())?;

    sink.sleep_until_end();
    Ok(())
}
