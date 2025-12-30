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
            gc.fill_color(color::get_color().to_owned());
            gc.fill();
        });
    }
}
