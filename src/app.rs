use std::time::Instant;

use crossterm::terminal;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::io;

use crate::config;
use crate::cube::Cube;
use crate::geometry::{Camera, Viewport};
use crate::input::{Action, InputHandler};
use crate::raster::Renderer;
use crate::terminal::FrameWriter;

type TermResult<T> = io::Result<T>;

pub struct App {
    cube: Cube,
    camera: Camera,
    renderer: Renderer,
    input: InputHandler,
    frame_writer: FrameWriter,
    rng: ThreadRng,
    running: bool,
}

impl App {
    pub fn new(frame_writer: FrameWriter) -> Self {
        Self {
            cube: Cube::new(),
            camera: Camera::new(),
            renderer: Renderer::new(),
            input: InputHandler::new(),
            frame_writer,
            rng: thread_rng(),
            running: true,
        }
    }

    pub fn run(&mut self) -> TermResult<()> {
        let mut viewport = current_viewport()?;
        while self.running {
            let frame_start = Instant::now();
            self.process_input()?;
            let frame = self.renderer.render(&self.cube, &self.camera, viewport);
            self.frame_writer.blit(&frame)?;
            viewport = current_viewport()?;
            self.cap_frame_rate(frame_start);
        }
        Ok(())
    }

    fn process_input(&mut self) -> TermResult<()> {
        let actions = self.input.poll_actions()?;
        for action in actions {
            self.dispatch(action);
        }
        Ok(())
    }

    fn dispatch(&mut self, action: Action) {
        match action {
            Action::RotateCamera { d_theta, d_phi } => {
                self.camera.orbit(d_theta, d_phi);
            }
            Action::RollCamera(delta) => self.camera.roll(delta),
            Action::ZoomCamera(delta) => self.camera.zoom(delta),
            Action::TwistFace(mv) => self.cube.apply_move(mv),
            Action::Scramble => self.cube.scramble(config::SCRAMBLE_LENGTH, &mut self.rng),
            Action::Reset => self.cube.reset(),
            Action::Quit => self.running = false,
        }
    }

    fn cap_frame_rate(&self, frame_start: Instant) {
        let frame_time = config::frame_duration();
        if let Some(remaining) = frame_time.checked_sub(frame_start.elapsed()) {
            std::thread::sleep(remaining);
        }
    }
}

fn current_viewport() -> TermResult<Viewport> {
    let (width, height) = terminal::size()?;
    Ok(Viewport { width, height })
}
