use crate::visuals::{
    visualizer::Visualizer,
    colours
};

use crate::math::{
    matrix::Matrix2,
    vector::Vector2
};


/// # `DiablosTree`
/// Structure that displays and animates the Rotating Tree Fractal
pub struct DiablosTree {
    visualizer: Visualizer,
    growth_matrix_1: Matrix2,
    growth_matrix_2: Matrix2
}

impl DiablosTree {

    /// # `new`
    /// Initializes the Rotating Tree fractal visualizer
    pub fn new() -> DiablosTree {
        DiablosTree {
            visualizer: Visualizer::new(None), 
            growth_matrix_1: super::GROWTH_FACTOR * Matrix2::rotation(super::ANGLE),
            growth_matrix_2: super::GROWTH_FACTOR * Matrix2::rotation(-super::ANGLE)
        }
    }

    fn branch(&mut self, start: Vector2, end: Vector2, depth: usize) {

        // Reach depth, draw leaves
        if depth == super::DEPTH {
            self.visualizer.draw_circle(start, (end-start).norm(), colours::RED);
            return;
        }

        self.visualizer.draw_line(start, end, 
            match depth {
                x if x >= 0 && x < 6 => colours::LIGHT_THEME[x],
                _ => colours::WHITE
            }, 
            super::DEPTH - depth);

        let next_branch = end - start;

        self.branch(end, end + self.growth_matrix_1 * next_branch, depth + 1);
        self.branch(end, end + self.growth_matrix_2 * next_branch, depth + 1);
    }
    
    pub fn draw(&mut self) {
        let mut angle = super::ANGLE;
        let mut growth_factor = 0.1;
        while self.visualizer.window.is_open() {
            self.visualizer.clear(None);
            self.growth_matrix_1 = (0.25 * (growth_factor as f32).sin() + 0.5) * Matrix2::rotation(angle);
            self.growth_matrix_2 = (0.25 * (growth_factor as f32).sin() + 0.5) * Matrix2::rotation(-angle);

            self.branch(super::START_POINT, super::END_POINT, 0);
            angle += 0.05;
            growth_factor += 0.005;
            self.visualizer.apply_buffer();
        }

        self.visualizer.end();
    }
}