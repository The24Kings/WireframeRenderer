use flo_canvas::*;
use flo_draw::*;

use rand::*;

use core::f32;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

const CANVAS_HEIGHT: f32 = 800.0;
const CANVAS_WIDTH: f32 = 800.0;

static COLOR: OnceLock<Color> = OnceLock::new();

fn get_color<'a>() -> &'a Color {
    COLOR.get_or_init(|| {
        let mut rng = rand::rng();

        Color::Rgba(
            rng.random::<f32>(),
            rng.random::<f32>(),
            rng.random::<f32>(),
            1.0,
        )
    })
}

pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn screen(&self) -> Point2D {
        Point2D {
            x: self.x * (CANVAS_WIDTH / 2.0),
            y: self.y * (CANVAS_HEIGHT / 2.0),
        }
    }

    pub fn draw(&self, canvas: &DrawingTarget) {
        let size = 16.0;
        let center = size / 2.0;

        canvas.draw(|gc| {
            gc.new_path();
            gc.rect(
                self.x - center,
                self.y - center,
                self.x - center + size,
                self.y - center + size,
            );
            gc.fill_color(get_color().to_owned());
            gc.fill();
        });
    }
}

pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn translate_x(&self, delta: f32) -> Self {
        Self {
            x: self.x + delta,
            y: self.y,
            z: self.z,
        }
    }

    pub fn translate_y(&self, delta: f32) -> Self {
        Self {
            x: self.x,
            y: self.y + delta,
            z: self.z,
        }
    }

    pub fn translate_z(&self, delta: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + delta,
        }
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        let c = f32::cos(angle);
        let s = f32::sin(angle);

        Self {
            x: self.x,
            y: self.y * c - self.z * s,
            z: self.y * s + self.z * c,
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        let c = f32::cos(angle);
        let s = f32::sin(angle);

        Self {
            x: self.x * c - self.z * s,
            y: self.y,
            z: self.x * s + self.z * c,
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        let c = f32::cos(angle);
        let s = f32::sin(angle);

        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
            z: self.z,
        }
    }

    fn project(&self) -> Point2D {
        match self.z {
            0.0 => Point2D::new(0.0, 0.0),
            _ => Point2D::new(self.x / self.z, self.y / self.z),
        }
    }
}

pub fn line(p1: &Point2D, p2: &Point2D, canvas: &DrawingTarget) {
    canvas.draw(|gc| {
        gc.stroke_color(get_color().to_owned());
        gc.line_width(2.0);

        gc.new_path();
        gc.move_to(p1.x, p1.y);
        gc.line_to(p2.x, p2.y);
        gc.stroke();
    });
}

pub fn clear(layer: LayerId, canvas: &DrawingTarget) {
    let boundary = Color::Rgba(0.2, 0.2, 0.2, 1.0);

    canvas.draw(|gc| {
        gc.layer(layer);
        gc.clear_layer();
        gc.canvas_height(CANVAS_HEIGHT);

        gc.new_path();
        gc.rect(
            -(CANVAS_WIDTH / 2.0),
            -(CANVAS_HEIGHT / 2.0),
            CANVAS_WIDTH / 2.0,
            CANVAS_HEIGHT / 2.0,
        );
        gc.fill_color(boundary);
        gc.fill();
    });
}

const FPS: f32 = 60.0;
const DELTA_TIME: f32 = 1.0 / FPS;
const FRAME_TIME: u64 = 1_000_000_000 / FPS as u64;

pub fn main() {
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(|| {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Wireframe Renderer");

        // Clear the canvas to set a background color
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
        });

        let vs = vec![
            // Back Face
            Point3D::new(0.25, 0.25, 0.25),
            Point3D::new(-0.25, 0.25, 0.25),
            Point3D::new(-0.25, -0.25, 0.25),
            Point3D::new(0.25, -0.25, 0.25),
            // Front Face
            Point3D::new(0.25, 0.25, -0.25),
            Point3D::new(-0.25, 0.25, -0.25),
            Point3D::new(-0.25, -0.25, -0.25),
            Point3D::new(0.25, -0.25, -0.25),
        ];

        let fs = vec![
            vec![0, 1, 2, 3], // Back
            vec![4, 5, 6, 7], // Front
            vec![0, 4],       // Top Right
            vec![1, 5],       // Top Left
            vec![2, 6],       // Bottom Left
            vec![3, 7],       // Bottom Right
        ];

        // let fs: Vec<usize> = fs.into_iter().flatten().collect();

        // Animate them
        let dz = 1.0;
        let mut angle = 0.0;

        loop {
            // dz += 1.0 * DELTA_TIME;
            angle += f32::consts::PI * DELTA_TIME;

            clear(LayerId(0), &canvas);

            // for v in vs.iter() {
            //     v.rotate_xy(angle)
            //         .translate_z(dz)
            //         .project()
            //         .screen()
            //         .draw(&canvas);
            // }

            for f in &fs {
                for i in 0..f.len() {
                    let a = &vs[f[i]];
                    let b = &vs[f[(i + 1) % f.len()]]; // Wrap around

                    line(
                        &a.rotate_x(angle)
                            .rotate_y(angle)
                            .rotate_z(angle)
                            .translate_z(dz)
                            .project()
                            .screen(),
                        &b.rotate_x(angle)
                            .rotate_y(angle)
                            .rotate_z(angle)
                            .translate_z(dz)
                            .project()
                            .screen(),
                        &canvas,
                    )
                }
            }

            // Point2D::new(1.0, 0.25).screen().draw(&canvas);

            // Wait for the next frame
            thread::sleep(Duration::from_nanos(FRAME_TIME));
        }
    });
}
