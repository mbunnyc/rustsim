use crate::{camera::Camera, color::Color, triangle::Triangle, vec2::Vector2, vec3::Vector3, vertex::Vertex};

pub struct TriangleGen;

impl TriangleGen {
    
    // Helper function to create a floor rectangle from two points
    pub fn create_floor_rect(
        start: Vector2,
        end: Vector2,
        height: f32,
        color: Color,
    ) -> Vec<Triangle> {
        // Ensure we create vertices in the correct order regardless of input points
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        let min_z = start.y.min(end.y); // Note: using y component of Vector2 as z coordinate
        let max_z = start.y.max(end.y);

        // Create the four corners
        let bottom_left = Vector3::new(min_x, height, min_z);
        let bottom_right = Vector3::new(max_x, height, min_z);
        let top_left = Vector3::new(min_x, height, max_z);
        let top_right = Vector3::new(max_x, height, max_z);

        // Calculate texture coordinates based on size
        let width = max_x - min_x;
        let depth = max_z - min_z;
        let tex_scale = 1.0; // Adjust this to control texture tiling

        // Create two triangles
        vec![
            // First triangle (bottom-left triangle)
            Triangle::new(
                Vertex::new(&bottom_left, &Vector2::new(0.0, 0.0), &color),
                Vertex::new(&bottom_right, &Vector2::new(width * tex_scale, 0.0), &color),
                Vertex::new(&top_left, &Vector2::new(0.0, depth * tex_scale), &color),
            ),
            // Second triangle (top-right triangle)
            Triangle::new(
                Vertex::new(&bottom_right, &Vector2::new(width * tex_scale, 0.0), &color),
                Vertex::new(
                    &top_right,
                    &Vector2::new(width * tex_scale, depth * tex_scale),
                    &color,
                ),
                Vertex::new(&top_left, &Vector2::new(0.0, depth * tex_scale), &color),
            ),
        ]
    }
    
    pub fn create_wall(
        bottom_left: &Vector3,
        length: f32,
        height: f32,
        rotation_deg: f32,
        color: &Color,
    ) -> Vec<Triangle> {
        // Convert rotation from degrees to radians
        let rotation_rad = rotation_deg.to_radians();
    
        // Calculate the direction vector based on rotation
        let direction_x = rotation_rad.cos();
        let direction_z = rotation_rad.sin();
    
        // Calculate the four corners of the wall
        let bottom_right = Vector3::new(
            bottom_left.x + direction_x * length,
            bottom_left.y,
            bottom_left.z + direction_z * length,
        );
    
        let top_left = Vector3::new(bottom_left.x, bottom_left.y + height, bottom_left.z);
    
        let top_right = Vector3::new(bottom_right.x, bottom_right.y + height, bottom_right.z);
    
        // Calculate texture coordinates based on wall dimensions
        let tex_scale = 1.0; // Adjust this to control texture tiling
        let tex_width = length * tex_scale;
        let tex_height = height * tex_scale;

        // Create two triangles to form the wall, with vertices in counter-clockwise order
        // when viewed from the front of the wall
        vec![
            Triangle::new(
                Vertex::new(&bottom_right, &Vector2::new(tex_width, 0.0), color),
                Vertex::new(&top_left, &Vector2::new(0.0, tex_height), color), 
                Vertex::new(&top_right, &Vector2::new(tex_width, tex_height), color),
            ),
            Triangle::new(
                Vertex::new(&bottom_right, &Vector2::new(tex_width, 0.0), color),
                Vertex::new(&top_left, &Vector2::new(0.0, tex_height), color),
                Vertex::new(&bottom_left, &Vector2::new(0.0, 0.0), color),
            ),
        ]
    }
    pub fn create_3d_line(
        start: &Vector3,
        end: &Vector3,
        camera: &Camera,
        start_color: &Color,
        end_color: &Color,
        thickness: f32,
    ) -> Vec<Triangle> {
        // Get camera right vector for billboarding
        let forward = Vector3::normalize_v(&(camera.pointing_at - camera.pos));
        let right = Vector3::normalize_v(&forward.cross(&Vector3::new(0.0, 1.0, 0.0)));
        
        // Calculate half-thickness offset vector
        let half_thickness = thickness * 0.5;
        let offset = Vector3::scale(&right, half_thickness);

        // Calculate the four corners of the line quad
        let start_left = Vector3::subtract(start, &offset);
        let start_right = Vector3::add(start, &offset);
        let end_left = Vector3::subtract(end, &offset);
        let end_right = Vector3::add(end, &offset);

        // Create two triangles with color gradient
        vec![
            Triangle::new(
                Vertex::new(&start_right, &Vector2::new(1.0, 0.0), start_color),
                Vertex::new(&end_left, &Vector2::new(0.0, 1.0), end_color),
                Vertex::new(&end_right, &Vector2::new(1.0, 1.0), end_color),
            ),
            Triangle::new(
                Vertex::new(&start_right, &Vector2::new(1.0, 0.0), start_color),
                Vertex::new(&start_left, &Vector2::new(0.0, 0.0), start_color),
                Vertex::new(&end_left, &Vector2::new(0.0, 1.0), end_color),
            ),
        ]
    }
}
