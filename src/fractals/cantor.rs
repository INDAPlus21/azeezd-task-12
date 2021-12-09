use super::super::visuals::{
    visualizer::Visualizer,
    visualizer::{WIDTH},
    colours
};

use crate::math::vector::Vector2;

const RECURSION_DEPTH : usize = 8;
const STARTING_POINT : Vector2 = Vector2{x: 50.0, y:50.0} ;
const LINE_LENGTH : f32 = WIDTH - 2.0 * STARTING_POINT.x;
const Y_AXIS_STEP : f32 = 20.0;

/// # `CantorSet`
/// Visualizes the Cantor Set fractal
pub struct CantorsSet {
    visualizer: Visualizer
}

impl CantorsSet {

    /// # `new`
    /// Initialize the CantorSet Fractal
    pub fn new() -> CantorsSet {
        CantorsSet {
            visualizer: Visualizer::new(None)
        }
    }

    fn cantor(&mut self, mut coord: Vector2, len: f32, mut depth: usize) {
        if depth >= RECURSION_DEPTH {return;}

        self.visualizer.draw_line(coord, Vector2::new(coord.x + len, coord.y), colours::CERISE, 3);
        self.visualizer.apply_buffer();

        coord.y += Y_AXIS_STEP * depth as f32;
        depth += 1;
        
        self.cantor(coord, len / 3.0, depth);
        self.cantor(Vector2::new(coord.x  + len * 2.0 / 3.0, coord.y), len / 3.0, depth);
    } 

    /// # `draw`
    /// Draws the Cantor Set on the window
    pub fn draw(&mut self) {
        self.visualizer.apply_buffer();
        self.visualizer.left_pressed();
        self.cantor(STARTING_POINT, LINE_LENGTH, 0);

        self.visualizer.end();
    }
}