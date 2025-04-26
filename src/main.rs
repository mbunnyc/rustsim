use core::f32;
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;
const SCREEN_PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

#[derive(Copy, Clone, Debug)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

const WHITE: Color = Color::new(255, 255, 255, 255);

#[derive(Clone)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn subtract(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z)
    }

    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn dot(a: &Vector3, b: &Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(v: &Vector3) -> Vector3 {
        let len = v.length();
        if len > 0.0001 {
            Vector3::new(v.x / len, v.y / len, v.z / len)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

struct Rect {
    pos: Vector2,
    size: Vector2,
}

impl Rect {
    pub fn clamped_to(&self, limit: Rect) -> Rect {
        let mut size_diff = Vector2::new(0.0, 0.0);
        Rect {
            pos: Vector2::new(
                if self.pos.x < limit.pos.x {
                    size_diff.x = self.pos.x + limit.pos.x;
                    limit.pos.x
                } else {
                    self.pos.x
                },
                if self.pos.y < limit.pos.y {
                    size_diff.y = self.pos.y + limit.pos.y;
                    limit.pos.y
                } else {
                    self.pos.y
                },
            ),
            size: Vector2::new(
                if self.size.x < limit.size.x {
                    limit.size.x + size_diff.x
                } else {
                    self.size.x + size_diff.x
                },
                if self.size.y < limit.size.y {
                    limit.size.y + size_diff.y
                } else {
                    self.size.y + size_diff.y
                },
            ),
        }
    }
}

struct Vertex {
    pos: Vector3,
    texture_coord: Vector2,
    color: Color,
}

impl Vertex {
    pub fn new(pos: &Vector3, texture_coord: &Vector2, color: &Color) -> Self {
        Self {
            pos: Vector3 {
                x: pos.x,
                y: pos.y,
                z: pos.z,
            },
            texture_coord: Vector2 {
                x: texture_coord.x, y: texture_coord.y
            },
            color: Color {
                r: color.r, 
                g: color.g, 
                b: color.b, 
                a: color.a
            },
        }
    }
}

struct Triangle {
    v1: Vertex,
    v2: Vertex,
    v3: Vertex,
}

impl Triangle {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Self { v1, v2, v3 }
    }

    // Helper function to create a floor rectangle from two points
    pub fn create_floor_rect(start: Vector2, end: Vector2, height: f32, color: Color) -> Vec<Triangle> {
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
                Vertex::new(
                    &bottom_left,
                    &Vector2::new(0.0, 0.0),
                    &color,
                ),
                Vertex::new(
                    &bottom_right,
                    &Vector2::new(width * tex_scale, 0.0),
                    &color,
                ),
                Vertex::new(
                    &top_left,
                    &Vector2::new(0.0, depth * tex_scale),
                    &color,
                ),
            ),
            // Second triangle (top-right triangle)
            Triangle::new(
                Vertex::new(
                    &bottom_right,
                    &Vector2::new(width * tex_scale, 0.0),
                    &color,
                ),
                Vertex::new(
                    &top_right,
                    &Vector2::new(width * tex_scale, depth * tex_scale),
                    &color,
                ),
                Vertex::new(
                    &top_left,
                    &Vector2::new(0.0, depth * tex_scale),
                    &color,
                ),
            ),
        ]
    }

    pub fn fill(&self, screen: &mut Screen, shader: &dyn PixelShader) {
        let min_x = self.v1.pos.x.min(self.v2.pos.x).min(self.v3.pos.x).floor() as usize;
        let max_x = self.v1.pos.x.max(self.v2.pos.x).max(self.v3.pos.x).ceil() as usize;
        let min_y = self.v1.pos.y.min(self.v2.pos.y).min(self.v3.pos.y).floor() as usize;
        let max_y = self.v1.pos.y.max(self.v2.pos.y).max(self.v3.pos.y).ceil() as usize;

        let min_x = min_x.clamp(0, SCREEN_WIDTH - 1);
        let max_x = max_x.clamp(0, SCREEN_WIDTH - 1);
        let min_y = min_y.clamp(0, SCREEN_HEIGHT - 1);
        let max_y = max_y.clamp(0, SCREEN_HEIGHT - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;

                let (alpha, beta, gamma) = self.barycentric_coords(px, py);

                if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
                    let color = self.interpolate_color(alpha, beta, gamma);
                    let pixel = PixelPlacement { x, y, color };
                    screen.draw_pixel(&pixel, shader);
                }
            }
        }
    }

    fn barycentric_coords(&self, px: f32, py: f32) -> (f32, f32, f32) {
        let v0 = Vector2::new(self.v2.pos.x - self.v1.pos.x, self.v2.pos.y - self.v1.pos.y);
        let v1 = Vector2::new(self.v3.pos.x - self.v1.pos.x, self.v3.pos.y - self.v1.pos.y);
        let v2 = Vector2::new(px - self.v1.pos.x, py - self.v1.pos.y);
        
        let d00 = v0.x * v0.x + v0.y * v0.y;
        let d01 = v0.x * v1.x + v0.y * v1.y;
        let d11 = v1.x * v1.x + v1.y * v1.y;
        let d20 = v2.x * v0.x + v2.y * v0.y;
        let d21 = v2.x * v1.x + v2.y * v1.y;
        
        let denom = d00 * d11 - d01 * d01;
        
        let beta = (d11 * d20 - d01 * d21) / denom;
        let gamma = (d00 * d21 - d01 * d20) / denom;
        let alpha = 1.0 - beta - gamma;
        
        (alpha, beta, gamma)
    }
    

    fn interpolate_color(&self, alpha: f32, beta: f32, gamma: f32) -> Color {
        let r = (self.v1.color.r as f32 * alpha
            + self.v2.color.r as f32 * beta
            + self.v3.color.r as f32 * gamma) as u8;

        let g = (self.v1.color.g as f32 * alpha
            + self.v2.color.g as f32 * beta
            + self.v3.color.g as f32 * gamma) as u8;

        let b = (self.v1.color.b as f32 * alpha
            + self.v2.color.b as f32 * beta
            + self.v3.color.b as f32 * gamma) as u8;

        Color { r, g, b, a: 255 }
    }

    pub fn project_and_fill(&self, screen: &mut Screen, camera: &Camera, shader: &dyn PixelShader) {
        let projected_triangle = self.with_applied_perspective(camera, SCREEN_WIDTH, SCREEN_HEIGHT);

        projected_triangle.fill(screen, shader);
    }

    fn with_applied_perspective(
        &self,
        camera: &Camera,
        screen_width: usize,
        screen_height: usize,
    ) -> Triangle {
        let forward = Vector3::normalize(&Vector3::subtract(&camera.pointing_at, &camera.pos));
        let right = Vector3::normalize(&Vector3::cross(&forward, &Vector3::new(0.0, 1.0, 0.0)));
        let up = Vector3::cross(&right, &forward);

        let aspect_ratio = screen_width as f32 / screen_height as f32;

        let fov_radians = camera.fov.to_radians();
        let tan_half_fov = (fov_radians / 2.0).tan();

        let project_vertex = |vertex: &Vertex| -> Vertex {
            let relative_pos = Vector3::subtract(&vertex.pos, &camera.pos);

            let camera_x = Vector3::dot(&relative_pos, &right);
            let camera_y = Vector3::dot(&relative_pos, &up);
            let camera_z = Vector3::dot(&relative_pos, &forward);

            let z_factor = if camera_z > 0.01 {
                1.0 / camera_z
            } else {
                100.0
            };

            let screen_x = (camera_x / (tan_half_fov * camera_z)) * 0.5 + 0.5;
            let screen_y = (camera_y / (tan_half_fov * camera_z / aspect_ratio)) * 0.5 + 0.5;

            let screen_x = screen_x * screen_width as f32;
            let screen_y = (1.0 - screen_y) * screen_height as f32;

            Vertex::new(
                &Vector3::new(screen_x, screen_y, z_factor),
                &vertex.texture_coord,
                &vertex.color,
            )
        };

        let projected_v1 = project_vertex(&self.v1);
        let projected_v2 = project_vertex(&self.v2);
        let projected_v3 = project_vertex(&self.v3);

        Triangle::new(projected_v1, projected_v2, projected_v3)
    }
}

struct Camera {
    pub fov: f32,
    pub pos: Vector3,
    pub pointing_at: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov: 45.0,
            pos: Vector3::new(7.0, 5.0, 2.0),
            pointing_at: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

struct Screen {
    pub pixels: Vec<Color>,
}

impl Screen {
    pub fn copy(&mut self, src: Rect, dst: Rect, to: &mut Screen) {
        // Calculate actual dimensions based on source and destination
        let src_width = src.size.x.floor() as usize;
        let src_height = src.size.y.floor() as usize;
        let src_x = src.pos.x.floor() as usize;
        let src_y = src.pos.y.floor() as usize;
        let dst_x = dst.pos.x.floor() as usize;
        let dst_y = dst.pos.y.floor() as usize;

        // Ensure we don't copy outside screen bounds
        let copy_width = src_width.min(SCREEN_WIDTH - dst_x).min(SCREEN_WIDTH - src_x);
        let copy_height = src_height.min(SCREEN_HEIGHT - dst_y).min(SCREEN_HEIGHT - src_y);

        // Copy pixels row by row
        for y in 0..copy_height {
            for x in 0..copy_width {
                let src_index = (src_y + y) * SCREEN_WIDTH + (src_x + x);
                let dst_index = (dst_y + y) * SCREEN_WIDTH + (dst_x + x);
                
                // Bounds check to prevent any possible overflow
                if src_index < SCREEN_PIXEL_COUNT && dst_index < SCREEN_PIXEL_COUNT {
                    to.pixels[dst_index] = self.pixels[src_index];
                }
            }
        }
    }
}

trait Window {
    fn start(&self, screen: &mut Screen, game: &mut dyn Game);
}

struct SDL2Window;

trait Game {
    fn update_tick(&mut self);
    fn render_tick(&self, screen: &mut Screen);
}

struct TestGame {
    cam: Camera,
}

impl Game for TestGame {
    fn update_tick(&mut self) {}
    fn render_tick(&self, screen: &mut Screen) {
        let sh = DummyPassthruShader;
        // Create a 2x3 floor at height 0
        let floor_tris = Triangle::create_floor_rect(
            Vector2::new(-1.0, -1.5),
            Vector2::new(1.0, 1.5),
            0.0,
            Color::new(128, 128, 0, 255),
        );

        // Draw both triangles that make up the floor
        for triangle in floor_tris {
            screen.draw_triangle(&triangle, &self.cam, &sh);
        }

    }
}

impl Window for SDL2Window {
    fn start(&self, screen: &mut Screen, game: &mut dyn Game) {
        let sdl_context: Sdl = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rustsim", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, 640, 480)
            .unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            game.update_tick();
            game.render_tick(screen);
            texture
                .with_lock(None, |pixels: &mut [u8], pitch: usize| {
                    for y in 0..SCREEN_HEIGHT {
                        for x in 0..SCREEN_WIDTH {
                            let i = y * SCREEN_WIDTH + x;
                            let col = screen.pixels[i];
                            let offset = y * pitch + x * 4;
                            pixels[offset] = col.b;
                            pixels[offset + 1] = col.g;
                            pixels[offset + 2] = col.r;
                            pixels[offset + 3] = col.a;
                        }
                    }
                })
                .unwrap();

            canvas.clear();
            canvas
                .copy(
                    &texture,
                    None,
                    Some(sdl2::rect::Rect::new(
                        0,
                        0,
                        SCREEN_WIDTH as u32,
                        SCREEN_HEIGHT as u32,
                    )),
                )
                .unwrap();
            canvas.present();

            for event in event_pump.poll_iter() {
                use sdl2::event::Event;
                if let Event::Quit { .. } = event {
                    break 'running;
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(1000 / 18));
        }
    }
}

struct PixelPlacement {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

trait PixelShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement;
}

struct DummyPassthruShader;

impl PixelShader for DummyPassthruShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement {
        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: pp.color,
        }
    }
}

struct EverythingIsRedShader;

impl PixelShader for EverythingIsRedShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement {
        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        }
    }
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: vec![Color::new(0, 200, 50, 255); SCREEN_PIXEL_COUNT],
        }
    }

    pub fn draw_pixel(&mut self, pp: &PixelPlacement, shader: &dyn PixelShader) {
        let processed_pp = shader.process(pp);
        self.pixels[processed_pp.y * SCREEN_WIDTH + processed_pp.x] = processed_pp.color;
    }

    pub fn draw_triangle(&mut self, tri: &Triangle, cam: &Camera, shader: &dyn PixelShader) {
        tri.project_and_fill(self, cam, shader);
    }
}

fn start_game(screen: &mut Screen, win: &dyn Window, game: &mut dyn Game) {
    win.start(screen, game);
}

fn main() {
    let mut screen = Screen::new();
    let win = SDL2Window;
    let mut game = TestGame { cam: Camera::new() };
    start_game(&mut screen, &win, &mut game);
}
