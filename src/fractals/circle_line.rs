use super::super::visuals::{
    visualizer::Visualizer,
    visualizer::{WIDTH, HEIGHT},
    colours
};

const MINIMUM_RADIUS : usize = 2;
const STARTING_RADIUS : usize = WIDTH / 4;
const STARTING_POINT : (usize, usize) = (WIDTH / 2, HEIGHT / 2);

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

    fn circle(&mut self, coord: (usize, usize), radius: usize) {
        // Draw circle
        self.visualizer.draw_circle(coord, radius, colours::CERISE);
        self.visualizer.apply_buffer();

        // Recursion if greater than a certain radius
        if radius > MINIMUM_RADIUS {
            self.circle((coord.0 - radius, coord.1), radius / 2);
            self.circle((coord.0 + radius, coord.1), radius / 2);
        }
    }


    pub fn draw(&mut self) {
        self.circle(STARTING_POINT, STARTING_RADIUS);

        self.visualizer.end();
    }
}
