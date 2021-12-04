use super::super::visuals::{
    visualizer::Visualizer,
    visualizer::{WIDTH},
    colours
};

const RECURSION_DEPTH : usize = 8;
const STARTING_POINT : (usize, usize) = (50, 50);
const LINE_LENGTH : usize = WIDTH - 2 * STARTING_POINT.0;
const Y_AXIS_STEP : usize = 20;

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

    fn cantor(&mut self, mut coord: (usize, usize), len: usize, mut depth: usize) {
        if depth >= RECURSION_DEPTH {return;}

        self.visualizer.draw_line(coord, (coord.0 + len, coord.1), colours::CERISE, 3);
        self.visualizer.apply_buffer();

        coord.1 += Y_AXIS_STEP * depth;
        depth += 1;

        self.cantor(coord, len / 3, depth);
        self.cantor((coord.0  + len * 2 / 3, coord.1), len / 3, depth);
    } 

    /// # `draw`
    /// Draws the Cantor Set on the window
    pub fn draw(&mut self) {

        self.cantor(STARTING_POINT, LINE_LENGTH, 0);

        self.visualizer.end();
    }
}