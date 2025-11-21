mod app;
mod config;
mod cube;
mod geometry;
mod input;
mod raster;
mod terminal;

use std::io;

use app::App;
use terminal::{FrameWriter, TerminalGuard};

fn main() {
    if let Err(err) = run_app() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run_app() -> io::Result<()> {
    let _guard = TerminalGuard::new()?;
    let frame_writer = FrameWriter::new();
    let mut app = App::new(frame_writer);
    app.run()
}
