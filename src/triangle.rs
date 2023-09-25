use ultraviolet::{Isometry3, Mat4, Vec2, Vec3};

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

    pub fn truncated(&self) -> Triangle2D {
        Triangle2D::new(
            self.v0.truncated(),
            self.v1.truncated(),
            self.v2.truncated(),
        )
    }

    pub fn truncated_include_z(&self) -> (Triangle2D, [f32; 3]) {
        (self.truncated(), [self.v0.z, self.v1.z, self.v2.z])
    }
}

impl From<stl_io::Triangle> for Triangle3D {
    fn from(value: stl_io::Triangle) -> Self {
        Self {
            v0: Vec3::new(
                value.vertices[0][0],
                value.vertices[0][1],
                value.vertices[0][3],
            ),
            v1: Vec3::new(
                value.vertices[1][0],
                value.vertices[1][1],
                value.vertices[1][3],
            ),
            v2: Vec3::new(
                value.vertices[2][0],
                value.vertices[2][1],
                value.vertices[2][3],
            ),
        }
    }
}
