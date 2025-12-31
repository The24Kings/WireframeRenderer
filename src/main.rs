use core::f32;
use flo_canvas::*;
use flo_draw::*;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use std::thread;
use std::time::Duration;

use crate::constants::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::point2d::Point2D;
use crate::shape::{Shape as _, cube::Cube, penger::Penger};

pub mod color;
pub mod constants;
pub mod point2d;
pub mod point3d;
pub mod shape;

pub fn queue_points<'a, I>(points: I, layer_id: LayerId, gc: &mut Vec<Draw>)
where
    I: IntoIterator<Item = Point2D>,
{
    gc.layer(layer_id);
    gc.fill_color(color::get_color().to_owned());

    for p in points {
        gc.new_path();
        gc.circle(p.x, p.y, 4.0);
        gc.fill();
    }
}

pub fn queue_lines<'a, I>(segments: I, layer_id: LayerId, gc: &mut Vec<Draw>)
where
    I: IntoIterator<Item = (&'a Point2D, &'a Point2D)>,
{
    gc.layer(layer_id);
    gc.stroke_color(color::get_color().to_owned());
    gc.line_width(2.0);

    gc.new_path();

    for (p1, p2) in segments {
        gc.move_to(p1.x, p1.y);
        gc.line_to(p2.x, p2.y);
    }

    gc.stroke();
}

pub fn clear_layer(layer: LayerId, gc: &mut Vec<Draw>) {
    gc.layer(layer);
    gc.clear_layer();
    gc.canvas_height(CANVAS_HEIGHT);
}

pub fn set_boundary(canvas: &DrawingTarget) {
    let boundary = Color::Rgba(0.2, 0.2, 0.2, 1.0);

    canvas.draw(|gc| {
        gc.layer(LayerId(0));
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

// Inspired by Tsoding: https://github.com/tsoding/formula
pub fn main() {
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(|| {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Wireframe Renderer");

        // Clear the canvas to set a background color
        canvas.draw(|gc| gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0)));
        set_boundary(&canvas);

        // If you want to see Penger, replace "Cube" with "Penger" or add your own shape in the shape folder
        let vs = Penger::vertices().expect("No vertices found.");
        let fs = Penger::indices().expect("No indices found.");

        // Animate them
        let dz = 1.0;
        let mut angle = 0.0;

        loop {
            // dz += 1.0 * DELTA_TIME;
            angle += 50.0 * f32::consts::PI * constants::DELTA_TIME;

            // TODO: It would be funny if it followed the mouse lol

            // Apply translation, rotation, (maybe scale) to all vertices
            let points: Vec<Point2D> = vs
                .par_iter()
                .map(|v| v.rotate_y(angle).translate_z(dz).project().screen())
                .collect();

            // Calculate all line segments
            let mut segments: Vec<(&Point2D, &Point2D)> = Vec::with_capacity(vs.len() * 2);

            for f in &fs {
                for i in 0..f.len() {
                    let a_idx = f[i];
                    let b_idx = f[(i + 1) % f.len()];
                    let a = &points[a_idx];
                    let b = &points[b_idx];

                    segments.push((a, b));
                }
            }

            // Queue all draw calls then request to render them at the same time to reduce flickering
            canvas.draw(|gc| {
                gc.start_frame();

                clear_layer(LayerId(1), gc);
                // clear_layer(LayerId(2), gc);

                queue_lines(segments.iter().copied(), LayerId(1), gc);
                // queue_points(points.clone(), LayerId(2), gc);

                gc.show_frame();
            });

            // Wait for the next frame
            thread::sleep(Duration::from_nanos(constants::FRAME_TIME));
        }
    });
}
