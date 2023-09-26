use crate::triangle::Triangle3D;
use stl_io::Vector;
use ultraviolet::{Isometry3, Rotor3, Vec2, Vec3};

pub struct Object {
    pub position: Vec3,
    pub rotation: Rotor3,

    pub vertices: Vec<Vec3>,
    pub triangles: Vec<[usize; 3]>,

    pub normals: Vec<Vec3>, // * indexed per vertice, not per face
    pub uv_coords: Vec<Vec2>,

    pub textures: Vec<image::DynamicImage>,
}

impl Object {
    pub fn load_from_stl(path: &str) -> Result<Self, std::io::Error> {
        fn stl_vector_to_vec3(vector: Vector<f32>) -> Vec3 {
            Vec3::new(vector[0], vector[1], vector[2])
        }

        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        let mesh = stl_io::read_stl(&mut file)?;
        
        let vertices = mesh
            .vertices
            .into_iter()
            .map(|v| stl_vector_to_vec3(v))
            .collect();

        let (triangles, normals) = mesh
            .faces
            .into_iter()
            .map(|f| (f.vertices, stl_vector_to_vec3(f.normal)))
            .unzip();

        Ok(
            Self {
                position: Vec3::zero(),
                rotation: Rotor3::identity(),
                vertices,
                triangles,
                normals,
                uv_coords: Vec::new(),
                textures: Vec::new(),
            }
        )
    }

    pub fn load_many_from_obj(path: &str) -> Result<Vec<Self>, tobj::LoadError> {
        let mut load_options = tobj::LoadOptions::default();
            load_options.triangulate = true;
            load_options.ignore_lines = true;
            load_options.ignore_points = true;

        let (models, materials) = tobj::load_obj(path, &load_options)?;
        let materials = materials?;

        let mut res = Vec::new();
        for model in models {
            let vertices = model.mesh.positions
                .chunks_exact(3)
                .map(|c| Vec3::new(c[0], c[1], c[2]))
                .collect();

            let triangles = model.mesh.indices
                .chunks_exact(3)
                .map(|c| [c[0] as usize, c[1] as usize, c[2] as usize])
                .collect();

            let normals_unsorted = model.mesh.normals
                .chunks_exact(3)
                .map(|c| Vec3::new(c[0], c[1], c[2]))
                .collect::<Vec<_>>();
            let normals = model.mesh.normal_indices
                .into_iter()
                .map(|i| normals_unsorted[i as usize])
                .collect();

            let uv_coords_unsorted = model.mesh.texcoords
                .chunks_exact(2)
                .map(|c| Vec2::new(c[0], c[1]))
                .collect::<Vec<_>>();
            let uv_coords = model.mesh.texcoord_indices
                .into_iter()
                .map(|i| uv_coords_unsorted[i as usize])
                .collect();

            // TODO: Implement .mtl materials, which use Lambertian shading:
            // ? https://paulbourke.net/dataformats/mtl/
            // ? https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/diffuse-lambertian-shading.html
            
            let mut textures = Vec::new();
            if let Some(mat_idx) = model.mesh.material_id {
                if let Some(material) = materials.get(mat_idx) {
                    if let Some(texture_path) = &material.diffuse_texture {
                        if let Ok(reader) = image::io::Reader::open(texture_path) {
                            if let Ok(texture) = reader.decode() {
                                textures.push(texture);
                            }
                        }
                    }
                }
            }

            res.push(
                Self {
                    position: Vec3::zero(),
                    rotation: Rotor3::identity(),
                    vertices,
                    triangles,
                    normals,
                    uv_coords,
                    textures,
                }
            );
        }

        return Ok(res);
    }

    pub fn get_triangle(&self, indices: [usize; 3]) -> Option<Triangle3D> {
        Some(Triangle3D::new(
            *self.vertices.get(indices[0])?,
            *self.vertices.get(indices[1])?,
            *self.vertices.get(indices[2])?,
        ))
    }

    pub fn get_triangle_unchecked(&self, indices: [usize; 3]) -> Triangle3D {
        Triangle3D::new(
            self.vertices[indices[0]],
            self.vertices[indices[1]],
            self.vertices[indices[2]],
        )
    }

    pub fn get_transform(&self) -> Isometry3 {
        Isometry3::new(self.position, self.rotation)
    }
}
