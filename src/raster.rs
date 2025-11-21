use std::fmt::Write;

use crossterm::style::{Color, ResetColor, SetForegroundColor};

use crate::config;
use crate::cube::Cube;
use crate::geometry::{self, Camera, ProjectedFace, Vec2, Viewport};

pub struct Renderer {
    canvas: AsciiCanvas,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            canvas: AsciiCanvas::new(0, 0),
        }
    }

    pub fn render(&mut self, cube: &Cube, camera: &Camera, viewport: Viewport) -> Frame {
        if viewport.width == 0 || viewport.height == 0 {
            return Frame::empty();
        }

        self.canvas.ensure_size(viewport);
        self.canvas.clear();

        let faces = geometry::project_cube(cube, camera, viewport);
        for face in faces {
            self.draw_face(&face);
        }

        self.canvas.to_frame()
    }

    fn draw_face(&mut self, face: &ProjectedFace) {
        let ch = shade_to_char(face.brightness);
        let color = config::face_color_to_ansi(face.color);
        self.fill_triangle(
            face.points[0],
            face.points[1],
            face.points[2],
            face.depth,
            ch,
            Some(color),
        );
        self.fill_triangle(
            face.points[0],
            face.points[2],
            face.points[3],
            face.depth,
            ch,
            Some(color),
        );
    }

    fn fill_triangle(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        depth: f32,
        ch: char,
        color: Option<Color>,
    ) {
        let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as i32;
        let max_x = a.x.max(b.x).max(c.x).ceil() as i32;
        let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
        let max_y = a.y.max(b.y).max(c.y).ceil() as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                if inside_triangle(p, a, b, c) {
                    self.canvas.plot(x as usize, y as usize, depth, ch, color);
                }
            }
        }
    }
}

fn shade_to_char(brightness: f32) -> char {
    let ramp = config::ASCII_SHADES;
    let idx = (brightness.clamp(0.0, 1.0) * (ramp.len() as f32 - 1.0)).round() as usize;
    ramp[idx]
}

fn inside_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let ab = cross_z(a, b, p);
    let bc = cross_z(b, c, p);
    let ca = cross_z(c, a, p);
    (ab >= 0.0 && bc >= 0.0 && ca >= 0.0) || (ab <= 0.0 && bc <= 0.0 && ca <= 0.0)
}

fn cross_z(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub color: Option<Color>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            color: None,
        }
    }
}

pub struct Frame {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Frame {
    pub fn empty() -> Self {
        Self {
            width: 0,
            height: 0,
            cells: Vec::new(),
        }
    }

    pub fn as_ansi_string(&self) -> String {
        let mut output = String::with_capacity(self.cells.len() * 2);
        let mut current_color: Option<Color> = None;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells[y * self.width + x];
                if cell.color != current_color {
                    if let Some(color) = cell.color {
                        let _ = write!(&mut output, "{}", SetForegroundColor(color));
                    } else {
                        let _ = write!(&mut output, "{}", ResetColor);
                    }
                    current_color = cell.color;
                }
                output.push(cell.ch);
            }
            if y + 1 < self.height {
                output.push('\r');
                output.push('\n');
            }
        }
        let _ = write!(&mut output, "{}", ResetColor);
        output
    }
}

impl Frame {
    fn from_canvas(canvas: &AsciiCanvas) -> Self {
        Self {
            width: canvas.width,
            height: canvas.height,
            cells: canvas.cells.clone(),
        }
    }
}

struct AsciiCanvas {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    depth: Vec<f32>,
}

impl AsciiCanvas {
    fn new(width: usize, height: usize) -> Self {
        let area = width * height;
        Self {
            width,
            height,
            cells: vec![Cell::default(); area],
            depth: vec![f32::INFINITY; area],
        }
    }

    fn ensure_size(&mut self, viewport: Viewport) {
        let width = viewport.width.max(1) as usize;
        let height = viewport.height.max(1) as usize;
        if self.width == width && self.height == height {
            return;
        }
        self.width = width;
        self.height = height;
        let area = width * height;
        self.cells = vec![Cell::default(); area];
        self.depth = vec![f32::INFINITY; area];
    }

    fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
        for depth in &mut self.depth {
            *depth = f32::INFINITY;
        }
    }

    fn plot(&mut self, x: usize, y: usize, depth: f32, ch: char, color: Option<Color>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        if depth < self.depth[idx] {
            self.depth[idx] = depth;
            self.cells[idx] = Cell { ch, color };
        }
    }

    fn to_frame(&self) -> Frame {
        Frame::from_canvas(self)
    }
}
