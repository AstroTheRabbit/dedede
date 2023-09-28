use std::collections::HashMap;

use ultraviolet::Vec2;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseButton, ScanCode, VirtualKeyCode},
    window::CursorGrabMode,
};

pub struct InputManager {
    held_keycodes: HashMap<VirtualKeyCode, bool>,
    held_scancodes: HashMap<ScanCode, bool>,
    held_mouse_buttons: HashMap<MouseButton, bool>,
    cursor_pos: Vec2,
    mouse_delta: Vec2,

    pub cursor_mode: CursorGrabMode,
    pub cursor_visible: bool,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            held_keycodes: HashMap::new(),
            held_scancodes: HashMap::new(),
            held_mouse_buttons: HashMap::new(),
            cursor_pos: Vec2::zero(),
            mouse_delta: Vec2::zero(),
            cursor_mode: CursorGrabMode::None,
            cursor_visible: true,
        }
    }

    pub fn handle_keyboard_input(&mut self, input: KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            match input.state {
                winit::event::ElementState::Pressed => {
                    self.held_keycodes.insert(keycode, true);
                }
                winit::event::ElementState::Released => {
                    self.held_keycodes.insert(keycode, false);
                }
            }
        } else {
            match input.state {
                winit::event::ElementState::Pressed => {
                    self.held_scancodes.insert(input.scancode, true);
                }
                winit::event::ElementState::Released => {
                    self.held_scancodes.insert(input.scancode, false);
                }
            }
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.held_mouse_buttons.insert(button, true);
            }
            ElementState::Released => {
                self.held_mouse_buttons.insert(button, false);
            }
        }
    }

    pub fn handle_cursor_movement(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_pos = Vec2::new(position.x as f32, position.y as f32);
    }

    pub fn handle_mouse_movement(&mut self, delta: (f64, f64)) {
        self.mouse_delta = Vec2::new(delta.0 as f32, delta.1 as f32);
    }

    pub fn is_keycode_held(&self, code: VirtualKeyCode) -> bool {
        if let Some(held) = self.held_keycodes.get(&code) {
            return *held;
        } else {
            return false;
        }
    }

    pub fn is_scancode_held(&self, code: ScanCode) -> bool {
        if let Some(held) = self.held_scancodes.get(&code) {
            return *held;
        } else {
            return false;
        }
    }

    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        if let Some(held) = self.held_mouse_buttons.get(&button) {
            return *held;
        } else {
            return false;
        }
    }

    pub fn use_mouse_delta(&mut self) -> Vec2 {
        std::mem::replace(&mut self.mouse_delta, Vec2::zero())
    }
}
