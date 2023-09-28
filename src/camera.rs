use crate::{
    aabb::AABB,
    triangle::{Triangle2D, Triangle3D},
};
use ultraviolet::{projection, Isometry3, Mat4, Rotor3, Vec2, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Rotor3,

    pub vertical_fov: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub projection_matrix: Mat4,

    pub screen_width: f32,
    pub screen_height: f32,
    pub screen_aabb: AABB,
}

impl Camera {
    pub fn new(
        position: Vec3,
        rotation: Rotor3,
        vertical_fov: f32,
        z_near: f32,
        z_far: f32,
        width: u32,
        height: u32,
    ) -> Self {
        let mut res = Self {
            position,
            rotation,
            vertical_fov,
            z_near,
            z_far,
            projection_matrix: Mat4::identity(),
            screen_width: 0.,
            screen_height: 0.,
            screen_aabb: AABB::new(0., 0., 0., 0.),
        };
        res.update_screen_dimensions(width, height);
        res
    }

    pub fn get_local_space_transform(&self) -> Isometry3 {
        Isometry3::new(self.position, self.rotation).inversed()
    }

    fn aspect_ratio(&self) -> f32 {
        self.screen_width / self.screen_height
    }

    pub fn update_screen_dimensions(&mut self, width: u32, height: u32) {
        self.screen_width = width as f32;
        self.screen_height = height as f32;

        self.screen_aabb = AABB::new(0., self.screen_width, 0., self.screen_height);
        // ? https://developer.nvidia.com/content/depth-precision-visualized
        self.projection_matrix = projection::perspective_reversed_z_vk(
            self.vertical_fov,
            self.aspect_ratio(),
            self.z_near,
            self.z_far,
        );
    }

    /// Projects a [`Triangle3D`] from the camera's local space into clip space, returning
    /// the projected [`Triangle2D`] as well as the z-depth of the triangle's 3 vertices.
    pub fn project_triangle(&mut self, triangle: Triangle3D) -> (Triangle2D, [f32; 3]) {
        let projected = triangle.apply_matrix(self.projection_matrix);
        let (cam_space, z_res) = projected.truncated_include_z();
        (self.projected_to_screen(cam_space), z_res)
    }

    /// Transforms a triangle from clip space to pixel coordinates.
    pub fn projected_to_screen(&self, triangle: Triangle2D) -> Triangle2D {
        let half_width = self.screen_width / 2.;
        let half_height = self.screen_height / 2.;

        Triangle2D::new(
            Vec2::new(
                half_width * (triangle.v0.x + 1.),
                half_height * (triangle.v0.y + 1.),
            ),
            Vec2::new(
                half_width * (triangle.v1.x + 1.),
                half_height * (triangle.v1.y + 1.),
            ),
            Vec2::new(
                half_width * (triangle.v2.x + 1.),
                half_height * (triangle.v2.y + 1.),
            ),
        )
    }
}
