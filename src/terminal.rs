use std::io;
use std::io::{Stdout, Write, stdout};

use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

use crate::raster::Frame;

type TermResult<T> = io::Result<T>;

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> TermResult<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
    }
}

pub struct FrameWriter {
    stdout: Stdout,
}

impl FrameWriter {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn blit(&mut self, frame: &Frame) -> TermResult<()> {
        let ansi = frame.as_ansi_string();
        execute!(self.stdout, cursor::MoveTo(0, 0))?;
        self.stdout.write_all(ansi.as_bytes())?;
        self.stdout.flush()?;
        Ok(())
    }
}
