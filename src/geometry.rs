use once_cell::sync::Lazy;

use crate::config;
use crate::cube::{AxisDir, Cube, FaceColor, FaceletDescriptor, LatticePoint, facelet_descriptors};

const CELL_SPACING: f32 = 0.7;
const TILE_SIZE: f32 = 0.38;
const NORMAL_BIAS: f32 = 0.03;
const LIGHT_DIR: Vec3 = Vec3::new(0.3, 0.9, 0.6);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        if len.abs() < f32::EPSILON {
            self
        } else {
            self * (1.0 / len)
        }
    }

    pub fn rotate_about(self, axis: Vec3, angle: f32) -> Vec3 {
        let k = axis.normalize();
        let cos = angle.cos();
        let sin = angle.sin();
        self * cos + k.cross(self) * sin + k * (k.dot(self) * (1.0 - cos))
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    pub fn aspect(&self) -> f32 {
        if self.height == 0 {
            1.0
        } else {
            self.width as f32 / self.height as f32
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ProjectedFace {
    pub points: [Vec2; 4],
    pub depth: f32,
    pub brightness: f32,
    pub color: FaceColor,
}

#[derive(Clone, Debug)]
struct FaceletMesh {
    corners: [Vec3; 4],
    center: Vec3,
    normal: Vec3,
}

static FACELET_MESHES: Lazy<Vec<FaceletMesh>> = Lazy::new(|| {
    facelet_descriptors()
        .iter()
        .map(|desc| build_mesh(desc))
        .collect()
});

fn build_mesh(desc: &FaceletDescriptor) -> FaceletMesh {
    let spec = desc.face.spec();
    let center = lattice_to_vec3(desc.coord);
    let normal = axis_dir_to_vec3(spec.normal).normalize();
    let right = axis_dir_to_vec3(spec.right).normalize();
    let up = axis_dir_to_vec3(spec.up).normalize();
    let half_tile = TILE_SIZE * 0.5;

    let offset = normal * NORMAL_BIAS;
    let corners = [
        center - right * half_tile + up * half_tile + offset,
        center + right * half_tile + up * half_tile + offset,
        center + right * half_tile - up * half_tile + offset,
        center - right * half_tile - up * half_tile + offset,
    ];

    FaceletMesh {
        corners,
        center: center + offset,
        normal,
    }
}

fn lattice_to_vec3(point: LatticePoint) -> Vec3 {
    Vec3::new(
        point.x as f32 * CELL_SPACING,
        point.y as f32 * CELL_SPACING,
        point.z as f32 * CELL_SPACING,
    )
}

fn axis_dir_to_vec3(axis: AxisDir) -> Vec3 {
    let sign = axis.dir as f32;
    match axis.axis {
        crate::cube::Axis::X => Vec3::new(sign, 0.0, 0.0),
        crate::cube::Axis::Y => Vec3::new(0.0, sign, 0.0),
        crate::cube::Axis::Z => Vec3::new(0.0, 0.0, sign),
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    theta: f32,
    phi: f32,
    roll: f32,
    radius: f32,
    target: Vec3,
    fov_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            theta: std::f32::consts::FRAC_PI_4,
            phi: std::f32::consts::FRAC_PI_6,
            roll: 0.0,
            radius: 3.0,
            target: Vec3::zero(),
            fov_y: 1.0,
        }
    }

    pub fn orbit(&mut self, d_theta: f32, d_phi: f32) {
        self.theta = (self.theta + d_theta) % (std::f32::consts::TAU);
        self.phi = (self.phi + d_phi).clamp(-1.2, 1.2);
    }

    pub fn roll(&mut self, delta: f32) {
        self.roll = (self.roll + delta).clamp(-std::f32::consts::PI, std::f32::consts::PI);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.radius =
            (self.radius + delta).clamp(config::CAMERA_MIN_RADIUS, config::CAMERA_MAX_RADIUS);
    }

    pub fn basis(&self) -> CameraBasis {
        let cos_phi = self.phi.cos();
        let sin_phi = self.phi.sin();
        let sin_theta = self.theta.sin();
        let cos_theta = self.theta.cos();

        let eye = Vec3::new(
            self.target.x + self.radius * cos_phi * sin_theta,
            self.target.y + self.radius * sin_phi,
            self.target.z + self.radius * cos_phi * cos_theta,
        );

        let forward = (self.target - eye).normalize();
        let mut right = forward.cross(Vec3::new(0.0, 1.0, 0.0));
        if right.length() < 0.001 {
            right = forward.cross(Vec3::new(0.0, 0.0, 1.0));
        }
        right = right.normalize();
        let mut up = right.cross(forward).normalize();

        if self.roll.abs() > f32::EPSILON {
            right = right.rotate_about(forward, self.roll);
            up = right.cross(forward).normalize();
        }

        CameraBasis {
            eye,
            forward,
            right,
            up,
            fov_y: self.fov_y,
        }
    }
}

pub struct CameraBasis {
    pub eye: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub fov_y: f32,
}

pub fn project_cube(cube: &Cube, camera: &Camera, viewport: Viewport) -> Vec<ProjectedFace> {
    let colors = cube.face_colors();
    let basis = camera.basis();
    let mut faces = Vec::with_capacity(64);

    for (idx, mesh) in FACELET_MESHES.iter().enumerate() {
        if !is_face_visible(mesh, &basis) {
            continue;
        }

        if let Some(projected) = project_mesh(mesh, colors[idx], &basis, viewport) {
            faces.push(projected);
        }
    }

    faces
}

fn is_face_visible(mesh: &FaceletMesh, basis: &CameraBasis) -> bool {
    let to_camera = (basis.eye - mesh.center).normalize();
    mesh.normal.dot(to_camera) > 0.0
}

fn project_mesh(
    mesh: &FaceletMesh,
    color: FaceColor,
    basis: &CameraBasis,
    viewport: Viewport,
) -> Option<ProjectedFace> {
    let mut projected = [Vec2::new(0.0, 0.0); 4];
    let mut total_depth = 0.0;
    for (i, corner) in mesh.corners.iter().enumerate() {
        let (pt, depth) = project_point(*corner, basis, viewport)?;
        projected[i] = pt;
        total_depth += depth;
    }
    let depth = total_depth / 4.0;
    let brightness = shade_face(mesh.normal);
    Some(ProjectedFace {
        points: projected,
        depth,
        brightness,
        color,
    })
}

fn project_point(point: Vec3, basis: &CameraBasis, viewport: Viewport) -> Option<(Vec2, f32)> {
    let relative = point - basis.eye;
    let x = relative.dot(basis.right);
    let y = relative.dot(basis.up);
    let z = relative.dot(basis.forward);

    if z <= 0.05 {
        return None;
    }

    let f = 1.0 / (0.5 * basis.fov_y).tan();
    let aspect = viewport.aspect().max(0.5);
    let ndc_x = (x * f) / (aspect * z);
    let ndc_y = (y * f) / z;

    let screen_x = ((ndc_x + 1.0) * 0.5) * (viewport.width.saturating_sub(1) as f32);
    let screen_y = ((1.0 - (ndc_y + 1.0) * 0.5) * (viewport.height.saturating_sub(1) as f32))
        .clamp(0.0, (viewport.height.saturating_sub(1)) as f32);

    Some((Vec2::new(screen_x, screen_y), z))
}

fn shade_face(normal: Vec3) -> f32 {
    let light = LIGHT_DIR.normalize();
    let intensity = normal.normalize().dot(light).max(0.0);
    0.2 + 0.8 * intensity
}
