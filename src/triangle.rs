use crate::edge_cross_product;
use ultraviolet::{Vec2, Vec3, Isometry3, Mat4};

#[derive(Debug, Clone, Copy)]
pub struct Triangle2D {
    pub v0: Vec2,
    pub v1: Vec2,
    pub v2: Vec2,
}

impl Triangle2D {
    pub fn new(v0: Vec2, v1: Vec2, v2: Vec2) -> Self {
        Self { v0, v1, v2 }
    }

    pub fn winding_order(&self) -> f32 {
        edge_cross_product(self.v0, self.v1, self.v2).signum()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle3D {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Triangle3D {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self { v0, v1, v2 }
    }

    pub fn apply_transform(&self, transform: Isometry3) -> Triangle3D {
        Triangle3D::new(
            transform.transform_vec(self.v0),
            transform.transform_vec(self.v1),
            transform.transform_vec(self.v2),
        )
    }

    pub fn apply_matrix(&self, matrix: Mat4) -> Triangle3D {
        Triangle3D::new(
            matrix.transform_point3(self.v0),
            matrix.transform_point3(self.v1),
            matrix.transform_point3(self.v2),
        )
    }

    pub fn projected_to_screen(&self, width: u32, height: u32) -> (Triangle2D, f32) {
        let half_width = width as f32 / 2.;
        let half_height = height as f32 / 2.;
        (
            Triangle2D::new(
                Vec2::new(half_width * (self.v0.x + 1.), half_height * (self.v0.y + 1.)),
                Vec2::new(half_width * (self.v1.x + 1.), half_height * (self.v1.y + 1.)),
                Vec2::new(half_width * (self.v2.x + 1.), half_height * (self.v2.y + 1.)),
            ),
            self.v0.z // * All of the projected triangle's points have the same z-depth (give or take floating-point inconsistencies).
        )
    }
}

impl From<stl_io::Triangle> for Triangle3D {
    fn from(value: stl_io::Triangle) -> Self {
        Self {
            v0: Vec3::new(value.vertices[0][0], value.vertices[0][1], value.vertices[0][3]),
            v1: Vec3::new(value.vertices[1][0], value.vertices[1][1], value.vertices[1][3]),
            v2: Vec3::new(value.vertices[2][0], value.vertices[2][1], value.vertices[2][3])
        }
    }
}