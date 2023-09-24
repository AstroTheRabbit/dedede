use crate::triangle::Triangle3D;
use stl_io::Vector;
use ultraviolet::{Rotor3, Vec3, Isometry3};

pub struct Object {
    pub position: Vec3,
    pub rotation: Rotor3,
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<[usize; 3]>,
    pub normals: Vec<Vec3>,
}

impl Object {
    pub fn load_from_stl(path: &str) -> Result<Self, std::io::Error> {
        fn stl_vector_to_vec3(vector: Vector<f32>) -> Vec3 {
            Vec3::new(vector[0], vector[1], vector[2])
        }

        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        let mesh = stl_io::read_stl(&mut file)?;
        let vertices = mesh.vertices.into_iter().map(|v| stl_vector_to_vec3(v)).collect();
        let (triangles, normals) = mesh.faces.into_iter().map(|f| (f.vertices, stl_vector_to_vec3(f.normal))).unzip();
        Ok(Self { position: Vec3::zero(), rotation: Rotor3::identity(), vertices, triangles, normals })
    }

    pub fn get_triangle(&self, indices: [usize; 3]) -> Option<Triangle3D> {
        Some(Triangle3D::new(*self.vertices.get(indices[0])?, *self.vertices.get(indices[1])?, *self.vertices.get(indices[2])?))
    }

    pub fn get_transform(&self) -> Isometry3 {
        Isometry3::new(self.position, self.rotation)
    }
}