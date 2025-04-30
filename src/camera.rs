use crate::{vec2::Vector2, vec3::Vector3};

pub struct Camera {
    pub fov: f32,
    pub pos: Vector3,
    pub pointing_at: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov: 45.0,
            pos: Vector3::new(7.0, 5.0, 8.0),
            pointing_at: Vector3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn zoom(&mut self, zoom_amount: f32) {
        // Get direction vector from camera to target
        let direction = (self.pointing_at - self.pos).normalize_v();
        
        // Move camera position along direction vector
        // Positive zoom_amount moves camera closer to target
        self.pos = self.pos + direction * zoom_amount;
    }        
    pub fn drag_move(&mut self, initial_screen_pos: &Vector2, current_screen_pos: &Vector2, screen_res: &Vector2) {
        // Calculate drag delta in screen coordinates
        let drag_delta = *current_screen_pos - *initial_screen_pos;
        
        // Scale movement based on screen resolution to maintain consistent speed
        let movement_scale = 0.03 * (screen_res.x.max(screen_res.y) / 1000.0);
        
        // Get camera direction vectors
        let forward = (self.pointing_at - self.pos).normalize_v();
        let right = forward.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize_v();
        let up = right.cross(&forward);

        // Calculate movement vector based on drag direction
        let movement = right * (-drag_delta.x * movement_scale) + 
                    up * (drag_delta.y * movement_scale);

        // Update camera position while maintaining look-at point
        self.pos = self.pos + movement;
        self.pointing_at = self.pointing_at + movement;
    }
    /*pub fn first_person_look(&mut self, mouse_delta: &Vector2, screen_res: &Vector2) {
        // Simple first person camera control - rotate around current position based on mouse movement
        let base_sensitivity = -0.003; // Base sensitivity value
        let sensitivity = base_sensitivity * (screen_res.x.max(screen_res.y) / 1000.0); // Scale by screen resolution
        
        // Calculate rotation angles from mouse movement
        let yaw = mouse_delta.x * sensitivity;
        let pitch = -mouse_delta.y * sensitivity;
        
        // Get current direction vector
        let mut direction = self.pointing_at - self.pos;
        
        // Rotate direction around Y axis for yaw
        let cos_yaw = yaw.cos();
        let sin_yaw = yaw.sin();
        direction = Vector3::new(
            direction.x * cos_yaw - direction.z * sin_yaw,
            direction.y,
            direction.x * sin_yaw + direction.z * cos_yaw
        );
        
        // Rotate direction around right vector for pitch
        //let right = direction.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize_v();
        let cos_pitch = pitch.cos();
        let sin_pitch = pitch.sin();
        direction = Vector3::new(
            direction.x * cos_pitch + direction.y * sin_pitch,
            -direction.x * sin_pitch + direction.y * cos_pitch,
            direction.z
        ).normalize_v();
        
        // Update pointing_at based on rotated direction
        self.pointing_at = self.pos + direction;
    }*/
}
