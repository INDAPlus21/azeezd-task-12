use super::super::visuals::{
    visualizer::Visualizer,
    visualizer::{WIDTH, HEIGHT},
    colours
};

use crate::math::vector::Vector2;

const MINIMUM_RADIUS : f32 = 2.0;
const STARTING_RADIUS : f32 = WIDTH / 4.0;
const STARTING_POINT : Vector2 = Vector2{x: WIDTH / 2.0, 
                                         y: HEIGHT / 2.0};

/// # `CircleLine`
/// Visualizes the Line of Circle Fractal
pub struct CircleLine {
    visualizer: Visualizer
}

impl CircleLine {
    /// # `new`
    /// Initializes the CircleLine fractal
    pub fn new() -> CircleLine {
        CircleLine {
            visualizer: Visualizer::new(None)
        }
    }

    fn circle(&mut self, coord: Vector2, radius: f32) {
        // Draw circle
        self.visualizer.draw_circle(coord, radius, colours::CERISE);
        self.visualizer.apply_buffer();

        // Recursion if greater than a certain radius
        if radius > MINIMUM_RADIUS {
            self.circle(Vector2::new(coord.x - radius, coord.y), radius / 2.0);
            self.circle(Vector2::new(coord.x + radius, coord.y), radius / 2.0);
        }
    }


    pub fn draw(&mut self) {
        self.circle(STARTING_POINT, STARTING_RADIUS);

        self.visualizer.end();
    }
}
