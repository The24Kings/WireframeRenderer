use flo_canvas::Color;
use rand::Rng as _;
use std::sync::OnceLock;

static COLOR: OnceLock<Color> = OnceLock::new();

pub fn get_color<'a>() -> &'a Color {
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
