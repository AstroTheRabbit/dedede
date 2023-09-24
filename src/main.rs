mod triangle;
mod aabb;
mod scene;
mod object;

use std::{num::NonZeroU32, f32::consts::PI};
use aabb::AABB;
use ultraviolet::{Vec2, Vec3, Rotor3};
use winit::{window::{WindowBuilder, Fullscreen}, event::{Event, WindowEvent, VirtualKeyCode, ElementState}};

fn main() {
    
    let mut torus = object::Object::load_from_stl("./test-models/torus.stl").unwrap();
    torus.position = 2. * Vec3::unit_x();
    torus.rotation = Rotor3::from_euler_angles(0., PI / 2., 0.);
    
    let suzanne_hd = object::Object::load_from_stl("./test-models/suzanne_hd.stl").unwrap();
    
    let mut scene = scene::Scene::new(vec![suzanne_hd, torus]);
    let colors = [
        //R           G          B
        combine_rgb(  0,   0,   0), //   black
        combine_rgb(100, 100, 100), //    grey
        combine_rgb(255, 255, 255), //   white
        // combine_rgb(255,   0,   0), //     red
        // combine_rgb(  0, 255,   0), //   green
        // combine_rgb(  0,   0, 255), //    blue
        // combine_rgb(255, 255,   0), //  yellow
        // combine_rgb(255,   0, 255), // magenta
        // combine_rgb(  0, 255, 255), //    teal
    ];

    let event_loop = winit::event_loop::EventLoop::new();
    let window = WindowBuilder::new()
        .with_active(true)
        .with_title("DeDeDe")
        // .with_inner_size(LogicalSize::new(800, 400))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .expect("Unable to create window!");

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(id) if id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                let screen_aabb = AABB::from_points(Vec2::zero(), Vec2::new(width as f32, height as f32));

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                
                let mut tris = scene.project_objects(width, height);
                tris.sort_by(|a, b| a.1.total_cmp(&b.1));
                
                // TODO: https://codeplea.com/triangular-interpolation - https://www.desmos.com/calculator/ovebiysjce
                for (tri, _z) in tris {
                    let aabb = AABB::from(&tri).intersection(&screen_aabb);
                    let winding = tri.winding_order();

                    for p in aabb {
                        if screen_aabb.point_in_aabb(&p) {
                            let e01 = winding * edge_cross_product(p, tri.v0, tri.v1) > 0.;
                            let e12 = winding * edge_cross_product(p, tri.v1, tri.v2) > 0.;
                            let e20 = winding * edge_cross_product(p, tri.v2, tri.v0) > 0.;
                            let idx = p.y as usize * width as usize + p.x as usize;
                            
                            if e01 && e12 && e20 {
                                buffer[idx] = combine_rgb(255, 255, 255);
                            }
                            // else if buffer[idx] == 0 {
                            //     buffer[idx] = combine_rgb(100, 100, 100);
                            // }
                        }
                    }
                }
                
                buffer.present().unwrap();
                scene.objects[0].rotation = scene.objects[0].rotation * Rotor3::from_euler_angles(0., 0., 0.05);
            }

            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    },
                    WindowEvent::KeyboardInput { input, ..} => {
                        if input.state == ElementState::Pressed {
                            match input.virtual_keycode.unwrap() {
                                VirtualKeyCode::W => {
                                    scene.camera.position += 0.1 * Vec3::unit_z();
                                },
                                VirtualKeyCode::A => {
                                    scene.camera.position += 0.1 * Vec3::unit_x();
                                },
                                VirtualKeyCode::S => {
                                    scene.camera.position -= 0.1 * Vec3::unit_z();
                                },
                                VirtualKeyCode::D => {
                                    scene.camera.position -= 0.1 * Vec3::unit_x();
                                },
                                VirtualKeyCode::LShift => {
                                    scene.camera.position += 0.1 * Vec3::unit_y();
                                },
                                VirtualKeyCode::Space => {
                                    scene.camera.position -= 0.1 * Vec3::unit_y();
                                },
                                _ => {}
                            }
                        }
                    },
                    // WindowEvent::CursorMoved { device_id, position, modifiers } => todo!(),
                    // WindowEvent::CursorEntered { device_id } => todo!(),
                    // WindowEvent::CursorLeft { device_id } => todo!(),
                    // WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => todo!(),
                    // WindowEvent::MouseInput { device_id, state, button, modifiers } => todo!(),
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

pub fn edge_cross_product(p: Vec2, va: Vec2, vb: Vec2) -> f32 {
    let dp = p - va;
    let dv = vb - va;
    (dv.x * dp.y) - (dv.y * dp.x)
}

pub fn combine_rgb(r: u32, g: u32, b: u32) -> u32 {
    r << 16 | g << 8 | b
}