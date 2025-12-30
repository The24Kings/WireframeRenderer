use flo_canvas::{DrawingTarget, GraphicsContext as _, GraphicsPrimitives as _};

use crate::color;
use crate::constants::{CANVAS_HEIGHT, CANVAS_WIDTH};

pub struct Point2D {
    pub x: f32,
    pub y: f32,
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

    pub fn point(&self, canvas: &DrawingTarget) {
        let radius = 4.0;

        canvas.draw(|gc| {
            gc.new_path();
            gc.circle(self.x, self.y, radius);
            gc.fill_color(color::get_color().to_owned());
            gc.fill();
        });
    }
}
