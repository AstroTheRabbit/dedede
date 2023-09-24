use crate::{object::Object, triangle::Triangle2D};
use ultraviolet::{transform::Isometry3, Rotor3, Vec3, Mat4, projection};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Rotor3,
    pub vertical_fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32
}

impl Camera {
    pub fn get_local_space_transform(&self) -> Isometry3 {
        Isometry3::new(self.position, self.rotation).inversed()
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        projection::perspective_vk(self.vertical_fov, self.aspect_ratio, self.z_near, self.z_far)
    }

    fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self { position: Vec3::new(0., 0., -3.), rotation: Rotor3::identity(), vertical_fov: 1.15, aspect_ratio: 1., z_near: 0.001, z_far: 1000. }
    }
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(objects: Vec<Object>) -> Self {
        Self { objects, camera: Camera::default() }
    }

    pub fn project_objects(&mut self, width: u32, height: u32) -> Vec<(Triangle2D, f32)> {
        self.camera.update_aspect_ratio(width, height);
        let camera_space_transform = self.camera.get_local_space_transform();
        let projection_matrix = self.camera.get_projection_matrix();
        let mut res = Vec::new();

        for object in self.objects.iter() {
            let object_transform = object.get_transform();
            for tri in object.triangles.iter() {
                let triangle = object.get_triangle(*tri).unwrap();
                let projection = triangle
                    .apply_transform(object_transform)
                    .apply_transform(camera_space_transform)
                    .apply_matrix(projection_matrix);

                if projection.v0.z > 1. && projection.v1.z > 1. && projection.v2.z > 1. {
                    res.push(projection.projected_to_screen(width, height));
                }

            }
        }
        return res;
    }
}