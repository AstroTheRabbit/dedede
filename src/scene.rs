use std::time::{Duration, Instant};

use crate::{aabb::AABB, camera::Camera, input_manager::InputManager, object::Object};
use softbuffer::Buffer;
use ultraviolet::{Rotor3, Vec3, Vec2};
use winit::{event::VirtualKeyCode, window::{Window, CursorGrabMode}};

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

    pub fn update(&mut self, window: &mut Window) {
        let delta_time = self.update_delta_time().as_millis() as f32;
        
        if self.input_manager.is_keycode_held(VirtualKeyCode::Escape) {
            self.input_manager.cursor_visible = true;
            self.input_manager.cursor_mode = CursorGrabMode::None;
        } else if self.input_manager.is_mouse_button_held(winit::event::MouseButton::Left) {
            self.input_manager.cursor_visible = false;
            self.input_manager.cursor_mode = CursorGrabMode::Locked;
        }
        window.set_cursor_visible(self.input_manager.cursor_visible);
        window.set_cursor_grab(self.input_manager.cursor_mode).unwrap();

        if self.input_manager.is_keycode_held(VirtualKeyCode::W) {
            self.camera.position +=
                0.001 * delta_time * Vec3::unit_z().rotated_by(self.camera.rotation);
        }
        if self.input_manager.is_keycode_held(VirtualKeyCode::S) {
            self.camera.position -=
                0.001 * delta_time * Vec3::unit_z().rotated_by(self.camera.rotation);
        }

        if self.input_manager.is_keycode_held(VirtualKeyCode::A) {
            self.camera.position +=
                0.001 * delta_time * Vec3::unit_x().rotated_by(self.camera.rotation);
        }
        if self.input_manager.is_keycode_held(VirtualKeyCode::D) {
            self.camera.position -=
                0.001 * delta_time * Vec3::unit_x().rotated_by(self.camera.rotation);
        }

        if self.input_manager.is_keycode_held(VirtualKeyCode::LControl) {
            self.camera.position +=
                0.001 * delta_time * Vec3::unit_y().rotated_by(self.camera.rotation);
        }
        if self.input_manager.is_keycode_held(VirtualKeyCode::Space) {
            self.camera.position -=
                0.001 * delta_time * Vec3::unit_y().rotated_by(self.camera.rotation);
        }

        if !self.input_manager.cursor_visible {
            let mouse_delta = self.input_manager.use_mouse_delta();
            let sensitivity = Vec2::new(0.004, 0.003);
            self.camera.rotation = Rotor3::from_rotation_xz(mouse_delta.x * sensitivity.x) * self.camera.rotation;
            // self.camera.rotation = Rotor3::from_rotation_yz(mouse_delta.y * sensitivity.y) * self.camera.rotation;
            self.camera.rotation.normalize();
        }
        
        // dbg!(self.camera.rotation);
    }

    pub fn render(&mut self, buffer: &mut Buffer, width: u32, height: u32) {
        self.camera.update_screen_dimensions(width, height);
        let camera_space_transform = self.camera.get_local_space_transform();
        let mut depth_buffer = vec![1.; width as usize * height as usize];

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
                
                if wd.is_normal() {
                    if let Some(tri_aabb) = AABB::from(&screen_tri).intersection(&self.camera.screen_aabb) {
                        for p in tri_aabb {
                            let w0 = ((v1.y - v2.y) * (p.x - v2.x) + (v2.x - v1.x) * (p.y - v2.y)) / wd;
                            let w1 = ((v2.y - v0.y) * (p.x - v2.x) + (v0.x - v2.x) * (p.y - v2.y)) / wd;
                            let w2 = 1. - w0 - w1;
                            
                            if w0 > 0. && w1 > 0. && w2 > 0. {
                                let pz = w0 * z_depth[0] + w1 * z_depth[1] + w2 * z_depth[2];
                                let idx = p.y as usize * width as usize + p.x as usize;

                                if pz > 0. && pz.abs() < depth_buffer[idx] {
                                    depth_buffer[idx] = pz;
                                    if w0 < 0.01 || w1 < 0.01 || w2 < 0.01 {
                                        buffer[idx] = 255 << 16;
                                    } else {
                                        buffer[idx] = 0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // let max_z = depth_buffer.iter().filter(|v| v.is_finite()).max_by(|a,b| a.total_cmp(b)).unwrap_or(&1.);
        // let min_z = depth_buffer.iter().filter(|v| v.is_finite()).min_by(|a,b| a.total_cmp(b)).unwrap_or(&0.);
        // for (i, p) in buffer.iter_mut().enumerate() {
        //     if *p == 0 {
        //         let z = depth_buffer[i];
        //         let v = 255 - (255. * (z - min_z) / (max_z - min_z)) as u32;
        //         *p = v;
        //     }
        // }
    }
}
