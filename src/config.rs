use std::time::Duration;

use crossterm::style::Color;

use crate::cube::FaceColor;

pub const TARGET_FPS: u64 = 30;
pub const SCRAMBLE_LENGTH: usize = 25;
pub const CAMERA_ROTATE_STEP: f32 = 0.14;
pub const CAMERA_ELEVATION_STEP: f32 = 0.1;
pub const CAMERA_ROLL_STEP: f32 = 0.06;
pub const CAMERA_ZOOM_STEP: f32 = 0.45;
pub const CAMERA_MIN_RADIUS: f32 = 2.8;
pub const CAMERA_MAX_RADIUS: f32 = 9.5;
pub const ASCII_SHADES: &[char; 10] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

pub fn frame_duration() -> Duration {
    Duration::from_micros(1_000_000 / TARGET_FPS)
}

pub fn input_poll_timeout() -> Duration {
    Duration::from_millis(0)
}

pub fn face_color_to_ansi(color: FaceColor) -> Color {
    match color {
        FaceColor::White => Color::White,
        FaceColor::Yellow => Color::Yellow,
        FaceColor::Red => Color::Red,
        FaceColor::Orange => Color::Rgb {
            r: 255,
            g: 140,
            b: 0,
        },
        FaceColor::Blue => Color::Blue,
        FaceColor::Green => Color::Green,
    }
}
