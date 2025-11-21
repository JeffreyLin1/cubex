use once_cell::sync::Lazy;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AxisDir {
    pub axis: Axis,
    pub dir: i8,
}

const fn axis_dir(axis: Axis, dir: i8) -> AxisDir {
    AxisDir { axis, dir }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LatticePoint {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl LatticePoint {
    pub const fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn set_component(&mut self, axis: Axis, value: i8) {
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
            Axis::Z => self.z = value,
        }
    }

    pub fn add_component(&mut self, axis: Axis, delta: i8) {
        match axis {
            Axis::X => self.x += delta,
            Axis::Y => self.y += delta,
            Axis::Z => self.z += delta,
        }
    }

    pub fn component(&self, axis: Axis) -> i8 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    Up,
    Down,
    Right,
    Left,
    Front,
    Back,
}

impl Face {
    pub fn default_color(self) -> FaceColor {
        match self {
            Face::Up => FaceColor::White,
            Face::Down => FaceColor::Yellow,
            Face::Right => FaceColor::Red,
            Face::Left => FaceColor::Orange,
            Face::Front => FaceColor::Green,
            Face::Back => FaceColor::Blue,
        }
    }

    pub const fn spec(self) -> FaceSpec {
        match self {
            Face::Up => FaceSpec {
                face: Face::Up,
                normal: axis_dir(Axis::Y, 1),
                up: axis_dir(Axis::Z, -1),
                right: axis_dir(Axis::X, 1),
            },
            Face::Down => FaceSpec {
                face: Face::Down,
                normal: axis_dir(Axis::Y, -1),
                up: axis_dir(Axis::Z, 1),
                right: axis_dir(Axis::X, 1),
            },
            Face::Right => FaceSpec {
                face: Face::Right,
                normal: axis_dir(Axis::X, 1),
                up: axis_dir(Axis::Y, 1),
                right: axis_dir(Axis::Z, -1),
            },
            Face::Left => FaceSpec {
                face: Face::Left,
                normal: axis_dir(Axis::X, -1),
                up: axis_dir(Axis::Y, 1),
                right: axis_dir(Axis::Z, 1),
            },
            Face::Front => FaceSpec {
                face: Face::Front,
                normal: axis_dir(Axis::Z, 1),
                up: axis_dir(Axis::Y, 1),
                right: axis_dir(Axis::X, 1),
            },
            Face::Back => FaceSpec {
                face: Face::Back,
                normal: axis_dir(Axis::Z, -1),
                up: axis_dir(Axis::Y, 1),
                right: axis_dir(Axis::X, -1),
            },
        }
    }

    pub fn all() -> &'static [Face] {
        static FACES: [Face; 6] = [
            Face::Up,
            Face::Down,
            Face::Right,
            Face::Left,
            Face::Front,
            Face::Back,
        ];
        &FACES
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FaceColor {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FaceSpec {
    pub face: Face,
    pub normal: AxisDir,
    pub up: AxisDir,
    pub right: AxisDir,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FaceletDescriptor {
    pub face: Face,
    pub coord: LatticePoint,
    pub row: u8,
    pub col: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FaceletKey {
    pub coord: LatticePoint,
    pub face: Face,
}

static FACELETS: Lazy<Vec<FaceletDescriptor>> = Lazy::new(|| {
    let mut output = Vec::with_capacity(54);
    for face in Face::all() {
        let spec = face.spec();
        for row in 0..3 {
            for col in 0..3 {
                let coord = coord_for(&spec, row, col);
                output.push(FaceletDescriptor {
                    face: spec.face,
                    coord,
                    row,
                    col,
                });
            }
        }
    }
    output
});

static FACELET_INDEX: Lazy<HashMap<FaceletKey, usize>> = Lazy::new(|| {
    FACELETS
        .iter()
        .enumerate()
        .map(|(idx, desc)| {
            (
                FaceletKey {
                    coord: desc.coord,
                    face: desc.face,
                },
                idx,
            )
        })
        .collect()
});

const GRID_COLS: [i8; 3] = [-1, 0, 1];
const GRID_ROWS: [i8; 3] = [1, 0, -1];

fn coord_for(spec: &FaceSpec, row: u8, col: u8) -> LatticePoint {
    let mut point = LatticePoint::zero();
    point.set_component(spec.normal.axis, spec.normal.dir);
    point.add_component(spec.right.axis, spec.right.dir * GRID_COLS[col as usize]);
    point.add_component(spec.up.axis, spec.up.dir * GRID_ROWS[row as usize]);
    point
}

pub fn facelet_descriptors() -> &'static [FaceletDescriptor] {
    &FACELETS
}

pub fn facelet_index(coord: LatticePoint, face: Face) -> usize {
    FACELET_INDEX
        .get(&FaceletKey { coord, face })
        .copied()
        .expect("invalid facelet lookup")
}

#[derive(Clone, Copy, Debug)]
pub enum RotationDir {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, Debug)]
struct MoveDef {
    axis: Axis,
    layer: i8,
    dir: RotationDir,
    turns: u8,
}
// she clone on my debug till i partialeq
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Move {
    U,
    UPrime,
    U2,
    D,
    DPrime,
    D2,
    R,
    RPrime,
    R2,
    L,
    LPrime,
    L2,
    F,
    FPrime,
    F2,
    B,
    BPrime,
    B2,
}

impl Move {
    pub fn all() -> &'static [Move] {
        static MOVES: [Move; 18] = [
            Move::U,
            Move::UPrime,
            Move::U2,
            Move::D,
            Move::DPrime,
            Move::D2,
            Move::R,
            Move::RPrime,
            Move::R2,
            Move::L,
            Move::LPrime,
            Move::L2,
            Move::F,
            Move::FPrime,
            Move::F2,
            Move::B,
            Move::BPrime,
            Move::B2,
        ];
        &MOVES
    }
    // she move on my self till i def
    fn def(self) -> MoveDef {
        use Move::*;
        match self {
            U => MoveDef {
                axis: Axis::Y,
                layer: 1,
                dir: RotationDir::Clockwise,
                turns: 1,
            },
            UPrime => MoveDef {
                axis: Axis::Y,
                layer: 1,
                dir: RotationDir::CounterClockwise,
                turns: 1,
            },
            U2 => MoveDef {
                axis: Axis::Y,
                layer: 1,
                dir: RotationDir::Clockwise,
                turns: 2,
            },
            D => MoveDef {
                axis: Axis::Y,
                layer: -1,
                dir: RotationDir::CounterClockwise,
                turns: 1,
            },
            DPrime => MoveDef {
                axis: Axis::Y,
                layer: -1,
                dir: RotationDir::Clockwise,
                turns: 1,
            },
            D2 => MoveDef {
                axis: Axis::Y,
                layer: -1,
                dir: RotationDir::CounterClockwise,
                turns: 2,
            },
            R => MoveDef {
                axis: Axis::X,
                layer: 1,
                dir: RotationDir::Clockwise,
                turns: 1,
            },
            RPrime => MoveDef {
                axis: Axis::X,
                layer: 1,
                dir: RotationDir::CounterClockwise,
                turns: 1,
            },
            R2 => MoveDef {
                axis: Axis::X,
                layer: 1,
                dir: RotationDir::Clockwise,
                turns: 2,
            },
            L => MoveDef {
                axis: Axis::X,
                layer: -1,
                dir: RotationDir::CounterClockwise,
                turns: 1,
            },
            LPrime => MoveDef {
                axis: Axis::X,
                layer: -1,
                dir: RotationDir::Clockwise,
                turns: 1,
            },
            L2 => MoveDef {
                axis: Axis::X,
                layer: -1,
                dir: RotationDir::CounterClockwise,
                turns: 2,
            },
            F => MoveDef {
                axis: Axis::Z,
                layer: 1,
                dir: RotationDir::Clockwise,
                turns: 1,
            },
            FPrime => MoveDef {
                axis: Axis::Z,
                layer: 1,
                dir: RotationDir::CounterClockwise,
                turns: 1,
            },
            F2 => MoveDef {
                axis: Axis::Z,
                layer: 1,
                dir: RotationDir::Clockwise,
                turns: 2,
            },
            B => MoveDef {
                axis: Axis::Z,
                layer: -1,
                dir: RotationDir::CounterClockwise,
                turns: 1,
            },
            BPrime => MoveDef {
                axis: Axis::Z,
                layer: -1,
                dir: RotationDir::Clockwise,
                turns: 1,
            },
            B2 => MoveDef {
                axis: Axis::Z,
                layer: -1,
                dir: RotationDir::CounterClockwise,
                turns: 2,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    stickers: Vec<FaceColor>,
}

impl Cube {
    pub fn new() -> Self {
        let stickers = FACELETS
            .iter()
            .map(|desc| desc.face.default_color())
            .collect();
        Self { stickers }
    }

    pub fn reset(&mut self) {
        for (idx, desc) in FACELETS.iter().enumerate() {
            self.stickers[idx] = desc.face.default_color();
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn is_solved(&self) -> bool {
        for face in Face::all() {
            let reference = face.default_color();
            for desc in FACELETS.iter().filter(|d| d.face == *face) {
                let idx = facelet_index(desc.coord, desc.face);
                if self.stickers[idx] != reference {
                    return false;
                }
            }
        }
        true
    }

    pub fn apply_move(&mut self, mv: Move) {
        let def = mv.def();
        for _ in 0..def.turns {
            self.rotate_layer(def.axis, def.layer, def.dir);
        }
    }

    pub fn scramble<R: Rng + ?Sized>(&mut self, len: usize, rng: &mut R) {
        let mut last_axis: Option<Axis> = None;
        for _ in 0..len {
            let choice = loop {
                let mv = *Move::all().choose(rng).expect("moves list not empty");
                let axis = mv.def().axis;
                if Some(axis) != last_axis {
                    break mv;
                }
            };
            last_axis = Some(choice.def().axis);
            self.apply_move(choice);
        }
    }

    pub fn face_colors(&self) -> &[FaceColor] {
        &self.stickers
    }
}

fn rotate_layer(stickers: &mut [FaceColor], axis: Axis, layer: i8, dir: RotationDir) {
    let mut updated = stickers.to_vec();
    for (idx, desc) in FACELETS.iter().enumerate() {
        if desc.coord.component(axis) == layer {
            let new_coord = rotate_coord(desc.coord, axis, dir);
            let new_face = rotate_face(desc.face, axis, dir);
            let target = facelet_index(new_coord, new_face);
            updated[target] = stickers[idx];
        }
    }
    stickers.copy_from_slice(&updated);
}

impl Cube {
    fn rotate_layer(&mut self, axis: Axis, layer: i8, dir: RotationDir) {
        rotate_layer(&mut self.stickers, axis, layer, dir);
    }
}

fn rotate_face(face: Face, axis: Axis, dir: RotationDir) -> Face {
    let normal = face_to_normal(face);
    let rotated_normal = rotate_vector(normal, axis, dir);
    face_from_normal(rotated_normal)
}

fn face_to_normal(face: Face) -> LatticePoint {
    match face {
        Face::Up => LatticePoint::new(0, 1, 0),
        Face::Down => LatticePoint::new(0, -1, 0),
        Face::Right => LatticePoint::new(1, 0, 0),
        Face::Left => LatticePoint::new(-1, 0, 0),
        Face::Front => LatticePoint::new(0, 0, 1),
        Face::Back => LatticePoint::new(0, 0, -1),
    }
}

fn face_from_normal(normal: LatticePoint) -> Face {
    match (normal.x, normal.y, normal.z) {
        (0, 1, 0) => Face::Up,
        (0, -1, 0) => Face::Down,
        (1, 0, 0) => Face::Right,
        (-1, 0, 0) => Face::Left,
        (0, 0, 1) => Face::Front,
        (0, 0, -1) => Face::Back,
        _ => panic!("invalid face normal {:?}", normal),
    }
}

fn rotate_coord(point: LatticePoint, axis: Axis, dir: RotationDir) -> LatticePoint {
    match axis {
        Axis::X => {
            let (new_y, new_z) = rotate_pair(point.y, point.z, dir);
            LatticePoint::new(point.x, new_y, new_z)
        }
        Axis::Y => {
            let (new_x, new_z) = rotate_pair(point.x, point.z, dir);
            LatticePoint::new(new_x, point.y, new_z)
        }
        Axis::Z => {
            let (new_x, new_y) = rotate_pair(point.x, point.y, dir);
            LatticePoint::new(new_x, new_y, point.z)
        }
    }
}

fn rotate_vector(vec: LatticePoint, axis: Axis, dir: RotationDir) -> LatticePoint {
    rotate_coord(vec, axis, dir)
}

fn rotate_pair(a: i8, b: i8, dir: RotationDir) -> (i8, i8) {
    match dir {
        RotationDir::Clockwise => (b, -a),
        RotationDir::CounterClockwise => (-b, a),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn inverse_moves_restore_state() {
        let mut cube = Cube::new();
        cube.apply_move(Move::R);
        cube.apply_move(Move::RPrime);
        assert!(cube.is_solved());
    }

    #[test]
    fn double_turn_equivalence() {
        let mut cube = Cube::new();
        cube.apply_move(Move::U2);
        let mut other = Cube::new();
        other.apply_move(Move::U);
        other.apply_move(Move::U);
        assert_eq!(cube.face_colors(), other.face_colors());
    }

    #[test]
    fn scramble_changes_state() {
        let mut cube = Cube::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        cube.scramble(20, &mut rng);
        assert!(!cube.is_solved());
    }
}
