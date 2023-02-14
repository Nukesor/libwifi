use anyhow::Result;
use clap::Parser;

mod device;
mod parse;

use device::get_capture;
use parse::handle_packet;

#[derive(Parser, Debug)]
#[command(name = "Demo", about = "Catch some stuff", author, version)]
pub struct CliArguments {
    /// The device you want to listen on (e.g. [wlan0, wlp3s0])
    pub device: String,
}

fn main() -> Result<()> {
    // Nicer panic for debugging
    better_panic::install();
    // Parse commandline option.
    let opt = CliArguments::parse();

    let mut capture = get_capture(&opt.device)?;

    while let Ok(packet) = capture.next_packet() {
        handle_packet(packet)?;
    }

    Ok(())
}
