use core::f32;
use flo_canvas::*;
use flo_draw::*;
use std::thread;
use std::time::Duration;

use crate::{
    constants::{CANVAS_HEIGHT, CANVAS_WIDTH},
    point2d::Point2D,
    shape::{Shape as _, cube::Cube},
};

pub mod color;
pub mod constants;
pub mod point2d;
pub mod point3d;
pub mod shape;

pub fn line(p1: &Point2D, p2: &Point2D, canvas: &DrawingTarget) {
    canvas.draw(|gc| {
        gc.stroke_color(color::get_color().to_owned());
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

pub fn main() {
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(|| {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Wireframe Renderer");

        // Clear the canvas to set a background color
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
        });

        let vs = Cube::vertices().expect("No vertices found.");
        let fs = Cube::indices().expect("No indices found.");

        // Animate them
        let dz = 1.0;
        let mut angle = 0.0;

        loop {
            // dz += 1.0 * DELTA_TIME;
            angle += f32::consts::PI * constants::DELTA_TIME;

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
            thread::sleep(Duration::from_nanos(constants::FRAME_TIME));
        }
    });
}
