use super::super::visuals::{
    visualizer::Visualizer,
    visualizer::{WIDTH, HEIGHT},
    colours
};

use crate::math::vector::Vector2;

const MINIMUM_RADIUS : f32 = 4.0;
const STARTING_RADIUS : f32 = 10.0 * WIDTH / 53.0;
const STARTING_POINT : Vector2 = Vector2{x: WIDTH / 2.0, 
                                         y: HEIGHT / 2.0};

/// # `CircleLine`
/// Visualizes the Line of Circle Fractal
pub struct CircleSpace {
    visualizer: Visualizer
}

impl CircleSpace {
    /// # `new`
    /// Initializes the CircleSpace fractal
    pub fn new() -> CircleSpace {
        CircleSpace {
            visualizer: Visualizer::new(Some(0))
        }
    }

    fn circle(&mut self, coord: Vector2, radius: f32) {
        // Draw circle
        self.visualizer.draw_circle(coord, radius, colours::RUST - colours::CERISE / radius as u32);
        self.visualizer.apply_buffer();

        // Recursion if greater than a certain radius
        if radius > MINIMUM_RADIUS {
            self.circle(Vector2::new(coord.x - radius, coord.y), radius / 2.0);
            self.circle(Vector2::new(coord.x + radius, coord.y), radius / 2.0);

            self.circle(Vector2::new(coord.x, coord.y - radius), radius / 2.0);
            self.circle(Vector2::new(coord.x, coord.y + radius), radius / 2.0);
        }
    }


    pub fn draw(&mut self) {
        self.circle(STARTING_POINT, STARTING_RADIUS);

        self.visualizer.end();
    }
}
