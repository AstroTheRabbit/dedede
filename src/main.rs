//   black - combine_rgb(  0,   0,   0)
//    grey - combine_rgb(100, 100, 100)
//   white - combine_rgb(255, 255, 255)
//     red - combine_rgb(255,   0,   0)
//   green - combine_rgb(  0, 255,   0)
//    blue - combine_rgb(  0,   0, 255)
//  yellow - combine_rgb(255, 255,   0)
// magenta - combine_rgb(255,   0, 255)
//    teal - combine_rgb(  0, 255, 255)

mod aabb;
mod camera;
mod input_manager;
mod object;
mod scene;
mod triangle;

use object::Object;
use ultraviolet::{Rotor3, Vec3};
use std::{num::NonZeroU32, f32::consts::PI};
use winit::{
    event::{Event, WindowEvent, DeviceEvent},
    window::{Fullscreen, WindowBuilder, CursorGrabMode},
};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut window = WindowBuilder::new()
        .with_active(true)
        .with_title("DeDeDe")
        // .with_inner_size(LogicalSize::new(800, 400))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .expect("Unable to create window!");

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    // let mut scene = scene::Scene::new(
    //     vec![Object::load_from_stl("test models/basics/suzanne_hd.stl").unwrap()],
    //     window.inner_size().width,
    //     window.inner_size().height,
    // );

    
    // let mut suzanne_uv = Object::load_many_from_obj("test models/uv mapping/suzanne_uv.obj").unwrap().pop().unwrap();
    // suzanne_uv.rotation = Rotor3::from_rotation_xy(PI);
    // let mut suzanne_hd = Object::load_from_stl("test models/basics/suzanne_hd.stl").unwrap();
    //     suzanne_hd.position += 3. * Vec3::unit_y();
    // let mut torus = Object::load_from_stl("test models/basics/torus.stl").unwrap();
    //     torus.position += 3. * Vec3::unit_x();
    let mut cube = Object::load_from_stl("test models/basics/cube.stl").unwrap();
    //     cube.position -= 3. * Vec3::unit_x();

    // let triangle = Object {
    //     position: Vec3::zero(),
    //     rotation: Rotor3::identity(),
    //     vertices: vec![Vec3::zero(), Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()],
    //     triangles: vec![[0,1,2],[0,1,3],[0,2,3]],
    //     normals: vec![],
    //     uv_coords: vec![],
    //     textures: vec![],
    // };

    
    let mut scene = scene::Scene::new(
        vec![
            // suzanne_uv,
            // suzanne_hd,
            // torus,
            cube,
            // triangle,
            ],
        window.inner_size().width,
        window.inner_size().height,
    );

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::MainEventsCleared | Event::RedrawRequested(_) => {
                scene.update(&mut window);

                let (width, height) = (window.inner_size().width, window.inner_size().height);
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                scene.render(&mut buffer, width, height);
                buffer.present().unwrap();
            }

            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        scene.input_manager.handle_keyboard_input(input)
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        scene.input_manager.handle_mouse_button(button, state)
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        scene.input_manager.handle_cursor_movement(position)
                    }
                    // WindowEvent::CursorEntered { device_id } => todo!(),
                    // WindowEvent::CursorLeft { device_id } => todo!(),
                    // WindowEvent::MouseWheel { device_id, delta, phase, .. } => todo!(),
                    _ => {}
                }
            }

            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::MouseMotion { delta } => {
                        scene.input_manager.handle_mouse_movement(delta);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
