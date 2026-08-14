use std::println;

use winit::{ keyboard::KeyCode };

use crate::camera;

use cgmath::Vector3;
use cgmath::InnerSpace;

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    // TODO: to move somewhere else
    yaw: f64,
    pitch: f64,
    direction: Vector3<f64>,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed:     false,
            is_backward_pressed:    false,
            is_left_pressed:        false,
            is_right_pressed:       false,
            yaw: -90.0f64,
            pitch: 0.0f64,
            direction: cgmath::Vector3::new(0.0f64, 0.0f64, 0.0f64),
        }
    }

    pub fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
        match code {
            KeyCode::KeyZ | KeyCode::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            }
            KeyCode::KeyQ | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn rotate_camera(&mut self, x: f64, y :f64) {

        println!("x: {}, y {}", x, y);

        let sensitivity = 0.003f64;

        // Add the motion values to the camera's yaw and pitch values
        self.yaw += x * sensitivity;
        self.pitch += -y * sensitivity;

        // Add some constraints to the minimum/maximum pitch values
        if self.pitch > 89.0f64 {
            self.pitch = 89.0f64;
        }
        if self.pitch < -89.0f64 {
            self.pitch = -89.0f64;
        }

        // Calculate the direction vector
        self.direction.x = cgmath::Angle::cos(cgmath::Rad(self.yaw)) * cgmath::Angle::cos(cgmath::Rad(self.pitch));
        self.direction.y = cgmath::Angle::sin(cgmath::Rad(self.pitch));
        self.direction.z = cgmath::Angle::sin(cgmath::Rad(self.yaw)) * cgmath::Angle::cos(cgmath::Rad(self.pitch));
        self.direction.normalize();
    }

    pub fn update_camera(&self, camera: &mut camera::Camera) {
        use cgmath::InnerSpace;
        // let forward = camera.look_at - camera.eye;
        let forward = camera.look_at;
        // let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            // camera.eye += forward_norm * self.speed;
            camera.eye += forward * self.speed;
        }
        if self.is_backward_pressed {
            // camera.eye -= forward_norm * self.speed;
            camera.eye -= forward * self.speed;
        }

        // let right = forward_norm.cross(camera.up);
        // Redo radius calc in case the forward/backward is pressed.
        // let forward = camera.look_at - camera.eye;
        // let forward = camera.look_at;
        // let forward_mag = forward.magnitude();

        camera.look_at = Vector3 { x: self.direction.x as f32, y: self.direction.y as f32, z: self.direction.z as f32};

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            // camera.eye = camera.look_at - (forward + right * self.speed).normalize() * forward_mag;

            camera.eye += cgmath::Vector3::cross(camera.look_at, camera.up).normalize() * self.speed;
        }
        if self.is_left_pressed {
            camera.eye -= cgmath::Vector3::cross(camera.look_at, camera.up).normalize() * self.speed;
            // camera.eye = camera.look_at - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}