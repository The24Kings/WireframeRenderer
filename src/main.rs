use flo_canvas::*;
use flo_draw::*;

use rand::*;

use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

static CANVAS_HEIGHT: f32 = 800.0;
static CANVAS_WIDTH: f32 = 800.0;

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

    fn screen(&self) -> Point2D {
        let half_width = CANVAS_WIDTH / 2.0;
        let half_height = CANVAS_HEIGHT / 2.0;

        Point2D {
            x: half_width + (half_width * self.x),
            y: half_height + (half_height * self.y),
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

    fn project(&self) -> Point2D {
        match self.z {
            0.0 => Point2D { x: 0.0, y: 0.0 },
            _ => Point2D {
                x: self.x / self.z,
                y: self.y / self.z,
            },
        }
    }
}

pub fn clear(layer: LayerId, canvas: &DrawingTarget) {
    let boundary = Color::Rgba(0.2, 0.2, 0.2, 1.0);

    canvas.draw(|gc| {
        gc.layer(layer);
        gc.clear_layer();
        gc.canvas_height(CANVAS_HEIGHT);
        gc.center_region(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

        gc.new_path();
        gc.rect(0.0, 0.0, CANVAS_HEIGHT, CANVAS_WIDTH);
        gc.fill_color(boundary);
        gc.fill();
    });
}

static FPS: u64 = 64;
static FRAME_TIME: u64 = 1_000_000_000 / FPS;

pub fn main() {
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(|| {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Wireframe Renderer");

        // Clear the canvas to set a background color
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
        });

        // Set the boundary color
        clear(LayerId(0), &canvas);

        // Animate them
        const DELTA_TIME: f32 = 1.0 / FPS as f32;
        let mut dz = 0.0;

        loop {
            dz += 1.0 * DELTA_TIME;

            clear(LayerId(1), &canvas);

            Point3D::new(0.5, 0.0, 1.0 + dz)
                .project()
                .screen()
                .draw(&canvas);

            // Wait for the next frame
            thread::sleep(Duration::from_nanos(FRAME_TIME));
        }
    });
}
