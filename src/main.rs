use flo_canvas::*;
use flo_draw::*;

use rand::*;

use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

static CANVAS_HEIGHT: f32 = 800.0;
static CANVAS_WIDTH: f32 = 800.0;

static FPS: u64 = 64;
static DELTA_TIME: u64 = 1_000_000_000 / FPS;
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

struct Point {
    size: f32,
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        let size = 16.0;

        // Correct the coordinate system so that (0, 0) is at the top left of the canvas
        Self {
            size,
            x: x - (size / 2.0),
            y: ((CANVAS_HEIGHT - size) - y) + (size / 2.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        todo!();
    }
}

fn project(point: &Point) -> Point {
    todo!();
}

fn screen(point: &Point) -> Point {
    todo!();
}

///
/// Bouncing ball example that uses sprites to improve performance
///
/// bounce.rs renders the paths every frame, so each circle has to be re-tessellated every time. This uses
/// sprites so that the paths are only tessellated once, which reduces the CPU requirements considerably.
///
pub fn main() {
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(|| {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Wireframe Renderer");
        let boundary = Color::Rgba(0.2, 0.2, 0.2, 1.0);

        // Clear the canvas to set a background color
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
        });

        // Set the boundary color
        canvas.draw(|gc| {
            gc.layer(LayerId(0));
            gc.clear_layer();
            gc.canvas_height(CANVAS_HEIGHT);
            gc.center_region(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
            gc.new_path();
            gc.rect(0.0, 0.0, CANVAS_HEIGHT, CANVAS_WIDTH);
            gc.fill_color(boundary);
            gc.fill();
        });

        // Generate some random balls
        let points = (0..1) // TODO: Load vertices
            .into_iter()
            .map(|_| Point::new(CANVAS_WIDTH / 2.0, CANVAS_HEIGHT / 2.0))
            .collect::<Vec<_>>();

        // Animate them
        loop {
            // // Update the balls for this frame
            // for pnt in points.iter_mut() {
            //     pnt.update();
            // }

            // Render the frame on layer 1
            canvas.draw(|gc| {
                gc.layer(LayerId(1));
                gc.clear_layer();
                gc.canvas_height(CANVAS_HEIGHT);
                gc.center_region(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);

                for pnt in points.iter() {
                    // // 3D to 2D shenanigans
                    // let pnt = screen(&project(&pnt));

                    // Draw the point
                    gc.new_path();
                    gc.rect(pnt.x, pnt.y, pnt.x + pnt.size, pnt.y + pnt.size);
                    gc.fill_color(get_color().to_owned());
                    gc.fill();
                }
            });

            // Wait for the next frame
            thread::sleep(Duration::from_nanos(DELTA_TIME));
        }
    });
}
