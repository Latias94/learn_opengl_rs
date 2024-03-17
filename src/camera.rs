use crate::window::Key;
use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CameraMovement {
    None,
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    // camera attributes
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,
    // euler angles
    pub yaw: f32,
    pub pitch: f32,
    // camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            position: glm::vec3(0.0, 0.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, 1.0, 0.0),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        };
        camera.update_camera_vectors();
        camera
    }
}

impl Camera {
    /// constructor with vectors
    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Camera {
            position,
            world_up: up,
            yaw,
            pitch,
            ..Default::default()
        };
        camera.update_camera_vectors();
        camera
    }

    /// constructor with scalar values
    pub fn new_with_scalar(
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        up_x: f32,
        up_y: f32,
        up_z: f32,
        yaw: f32,
        pitch: f32,
    ) -> Self {
        let position = glm::vec3(pos_x, pos_y, pos_z);
        let up = glm::vec3(up_x, up_y, up_z);
        Camera::new(position, up, yaw, pitch)
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.movement_speed = speed;
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.mouse_sensitivity = sensitivity;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    /// processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        match direction {
            CameraMovement::Forward => self.position += self.front * velocity,
            CameraMovement::Backward => self.position -= self.front * velocity,
            CameraMovement::Left => self.position -= self.right * velocity,
            CameraMovement::Right => self.position += self.right * velocity,
            CameraMovement::None => {}
        }
    }

    pub fn process_keyboard_with_key(&mut self, key: Key, delta_time: f32) {
        let direction = Camera::map_key_to_movement(key);
        self.process_keyboard(direction, delta_time);
    }

    /// processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    /// processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= y_offset;
        }
        if self.zoom <= 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom >= 45.0 {
            self.zoom = 45.0;
        }
    }

    fn update_camera_vectors(&mut self) {
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = glm::normalize(&front);
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }

    pub fn map_key_to_movement(key: Key) -> CameraMovement {
        match key {
            Key::W | Key::Up => CameraMovement::Forward,
            Key::S | Key::Down => CameraMovement::Backward,
            Key::A | Key::Left => CameraMovement::Left,
            Key::D | Key::Right => CameraMovement::Right,
            _ => CameraMovement::None,
        }
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }
}
