use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;

use crate::config;
use crate::cube::Move;

#[derive(Debug)]
pub enum Action {
    RotateCamera { d_theta: f32, d_phi: f32 },
    RollCamera(f32),
    ZoomCamera(f32),
    TwistFace(Move),
    Scramble,
    Reset,
    Quit,
}

type TermResult<T> = io::Result<T>;

pub struct InputHandler {
    pending_prime: bool,
    pending_double: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            pending_prime: false,
            pending_double: false,
        }
    }

    pub fn poll_actions(&mut self) -> TermResult<Vec<Action>> {
        let mut actions = Vec::new();
        while event::poll(poll_timeout())? {
            match event::read()? {
                Event::Key(key) => {
                    if let Some(action) = self.handle_key_event(key) {
                        if let Some(action) = action {
                            actions.push(action);
                        }
                    }
                }
                Event::Resize(_, _) => {
                    // ignore explicit resize events since we redraw each frame anyway
                }
                _ => {}
            }
        }
        Ok(actions)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<Option<Action>> {
        if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
            return None;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            if matches!(key.code, KeyCode::Char('c') | KeyCode::Char('d')) {
                return Some(Some(Action::Quit));
            }
        }

        Some(match key.code {
            KeyCode::Esc => Some(Action::Quit),
            KeyCode::Char(' ') => Some(Action::Scramble),
            KeyCode::Char('x') | KeyCode::Char('X') => Some(Action::Reset),
            KeyCode::Char('+') | KeyCode::Char('=') => {
                Some(Action::ZoomCamera(-config::CAMERA_ZOOM_STEP))
            }
            KeyCode::Char('-') | KeyCode::Char('_') => {
                Some(Action::ZoomCamera(config::CAMERA_ZOOM_STEP))
            }
            KeyCode::Char('q') => Some(Action::RollCamera(-config::CAMERA_ROLL_STEP)),
            KeyCode::Char('e') => Some(Action::RollCamera(config::CAMERA_ROLL_STEP)),
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => Some(Action::RotateCamera {
                d_theta: -config::CAMERA_ROTATE_STEP,
                d_phi: 0.0,
            }),
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                Some(Action::RotateCamera {
                    d_theta: config::CAMERA_ROTATE_STEP,
                    d_phi: 0.0,
                })
            }
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => Some(Action::RotateCamera {
                d_theta: 0.0,
                d_phi: config::CAMERA_ELEVATION_STEP,
            }),
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => Some(Action::RotateCamera {
                d_theta: 0.0,
                d_phi: -config::CAMERA_ELEVATION_STEP,
            }),
            KeyCode::Char('\'') => {
                self.pending_prime = true;
                None
            }
            KeyCode::Char('2') => {
                self.pending_double = true;
                None
            }
            KeyCode::Char(ch) => self.handle_move_char(ch),
            _ => None,
        })
    }

    fn handle_move_char(&mut self, ch: char) -> Option<Action> {
        let mut prime = self.pending_prime;
        let double = self.pending_double;

        if ch.is_ascii_uppercase() {
            prime = true;
        }

        let normalized = ch.to_ascii_lowercase();
        self.pending_prime = false;
        self.pending_double = false;

        let mv = parse_move_letter(normalized, prime, double)?;
        Some(Action::TwistFace(mv))
    }
}

fn poll_timeout() -> Duration {
    config::input_poll_timeout()
}

fn parse_move_letter(letter: char, prime: bool, double: bool) -> Option<Move> {
    use Move::*;
    Some(match (letter, prime, double) {
        ('u', _, true) => U2,
        ('u', true, false) => UPrime,
        ('u', false, false) => U,
        ('d', _, true) => D2,
        ('d', true, false) => DPrime,
        ('d', false, false) => D,
        ('r', _, true) => R2,
        ('r', true, false) => RPrime,
        ('r', false, false) => R,
        ('l', _, true) => L2,
        ('l', true, false) => LPrime,
        ('l', false, false) => L,
        ('f', _, true) => F2,
        ('f', true, false) => FPrime,
        ('f', false, false) => F,
        ('b', _, true) => B2,
        ('b', true, false) => BPrime,
        ('b', false, false) => B,
        _ => return None,
    })
}
