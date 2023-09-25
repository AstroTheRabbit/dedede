use std::time::{Duration, Instant};

use crate::{aabb::AABB, camera::Camera, input_manager::InputManager, object::Object};
use softbuffer::Buffer;
use ultraviolet::{Rotor3, Vec2, Vec3};
use winit::event::VirtualKeyCode;

pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
    pub input_manager: InputManager,
    prev_update_time: Instant,
}

impl Scene {
    pub fn new(objects: Vec<Object>, screen_width: u32, screen_height: u32) -> Self {
        Self {
            objects,
            camera: Camera::new(
                Vec3::new(0., 0., -5.),
                Rotor3::identity(),
                1.5,
                0.01,
                1000.,
                screen_width,
                screen_height,
            ),
            input_manager: InputManager::new(),
            prev_update_time: Instant::now(),
        }
    }

    pub fn update_delta_time(&mut self) -> Duration {
        let now = Instant::now();
        let prev = std::mem::replace(&mut self.prev_update_time, now.clone());
        now - prev
    }

    pub fn update(&mut self) {
        let delta_time = self.update_delta_time().as_millis() as f32;

        if self.input_manager.is_keycode_held(VirtualKeyCode::W) {
            self.camera.position +=
                0.001 * delta_time * Vec3::unit_z().rotated_by(self.camera.rotation);
        } else if self.input_manager.is_keycode_held(VirtualKeyCode::S) {
            self.camera.position -=
                0.001 * delta_time * Vec3::unit_z().rotated_by(self.camera.rotation);
        }

        if self.input_manager.is_keycode_held(VirtualKeyCode::A) {
            self.camera.position +=
                0.001 * delta_time * Vec3::unit_x().rotated_by(self.camera.rotation);
        } else if self.input_manager.is_keycode_held(VirtualKeyCode::D) {
            self.camera.position -=
                0.001 * delta_time * Vec3::unit_x().rotated_by(self.camera.rotation);
        }

        if self.input_manager.is_keycode_held(VirtualKeyCode::LControl) {
            self.camera.position +=
                0.001 * delta_time * Vec3::unit_y().rotated_by(self.camera.rotation);
        } else if self.input_manager.is_keycode_held(VirtualKeyCode::Space) {
            self.camera.position -=
                0.001 * delta_time * Vec3::unit_y().rotated_by(self.camera.rotation);
        }

        self.objects[0].rotation = self.objects[0].rotation * Rotor3::from_rotation_xz(0.01);
    }

    pub fn render(&mut self, buffer: &mut Buffer, width: u32, height: u32) {
        fn get_uv_test_color(uv: Vec2) -> u32 {
            ((255. * uv.x) as u32) << 16
                | ((127.5 * (uv.x + uv.y)) as u32) << 8
                | (255. * uv.y) as u32
        }
        let u0 = Vec2::new(1., 0.);
        let u1 = Vec2::new(0., 1.);
        let u2 = Vec2::new(1., 1.);

        self.camera.update_screen_dimensions(width, height);
        let camera_space_transform = self.camera.get_local_space_transform();
        let mut depth_buffer = vec![f32::INFINITY; width as usize * height as usize];

        for obj in &self.objects {
            let transform = obj.get_transform();

            for tri_indices in &obj.triangles {
                let tri = obj.get_triangle_unchecked(*tri_indices);
                let local_tri = tri
                    .apply_transform(transform)
                    .apply_transform(camera_space_transform);
                let (screen_tri, z_depth) = self.camera.project_triangle(local_tri);

                // ? Barycentric coordinates: https://www.desmos.com/calculator/ovebiysjce
                let v0 = screen_tri.v0;
                let v1 = screen_tri.v1;
                let v2 = screen_tri.v2;
                let wd = (v1.y - v2.y) * (v0.x - v2.x) + (v2.x - v1.x) * (v0.y - v2.y);

                if z_depth[0] < 1.
                    && z_depth[1] < 1.
                    && z_depth[2] < 1.
                    && z_depth[0] > 0.
                    && z_depth[1] > 0.
                    && z_depth[2] > 0.
                {
                    let tri_aabb = AABB::from(&screen_tri).intersection(&self.camera.screen_aabb);

                    for p in tri_aabb {
                        let idx = p.y as usize * width as usize + p.x as usize;
                        // TODO: Replace z_depth[0] with barycentric depth calc.
                        if z_depth[0] < depth_buffer[idx] {
                            let w0 =
                                ((v1.y - v2.y) * (p.x - v2.x) + (v2.x - v1.x) * (p.y - v2.y)) / wd;
                            let w1 =
                                ((v2.y - v0.y) * (p.x - v2.x) + (v0.x - v2.x) * (p.y - v2.y)) / wd;
                            let w2 = 1. - w0 - w1;

                            if w0.is_sign_positive()
                                && w1.is_sign_positive()
                                && w2.is_sign_positive()
                            {
                                depth_buffer[idx] = z_depth[0];
                                buffer[idx] = get_uv_test_color(w0 * u0 + w1 * u1 + w2 * u2);
                            } else {
                                buffer[idx] = 100 << 16 | 100 << 8 | 100;
                            }
                        }
                    }
                }
            }
        }
    }
}
