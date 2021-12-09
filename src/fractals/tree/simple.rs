use crate::visuals::{
    visualizer::{WIDTH, HEIGHT, Visualizer},
    colours
};
use minifb::{MouseButton, MouseMode};

use std::f32::consts::PI;

use crate::math::{
    matrix::Matrix2,
};

use super::Branch;

/// # `Simple Tree`
/// Structure that creates a very simple tree fractal
pub struct SimpleTree {
    visualizer: Visualizer,
    branches: Vec<Branch>,
    growth_matrix_1: Matrix2,
    growth_matrix_2: Matrix2,
}

impl SimpleTree {
    /// # `new`
    /// Initializes a new simple tree visualizer
    pub fn new() -> SimpleTree {
        SimpleTree {
            visualizer: Visualizer::new(None),
            branches: vec![Branch::new_stem()],

            // matrices are "rotate by some angle and scale length by some factor"
            growth_matrix_1: super::GROWTH_FACTOR * Matrix2::rotation(super::ANGLE),
            growth_matrix_2: super::GROWTH_FACTOR * Matrix2::rotation(-super::ANGLE)
        }
    }

    /// # `generate`
    /// Generates the next set of branches of the tree
    fn generate(&mut self) {
        let mut next: Vec<Branch> = Vec::with_capacity(self.branches.len());

        for branch in self.branches.iter() {
            next.push(branch.grow(self.growth_matrix_1));
            next.push(branch.grow(self.growth_matrix_2));
        }

        self.branches = next;
    }

    /// # `draw`
    /// Displays the fractal onto the window
    pub fn draw(&mut self) {

        // Since there is a while window is open loop, no need for visualizer.end() at the end of the draw function
        while self.visualizer.window.is_open() {
            // generating and drawing the tree
            for curr_depth in 0..super::DEPTH {
                for branch in self.branches.iter() {
                    self.visualizer.draw_line(branch.start, branch.end, colours::CERISE, super::DEPTH - curr_depth);
                }
                self.generate();
            }
            self.visualizer.apply_buffer();

            // Get mouse position and generate angle and growth based on the position
            let mouse_pos = self.visualizer.left_pressed();
            let angle = (mouse_pos.x / WIDTH - 0.5) * 2.0 * PI;
            let growth = 1.0 - mouse_pos.y / HEIGHT;

            // Generate matrices based on given angle or growth
            self.growth_matrix_1 = growth * Matrix2::rotation(angle);
            self.growth_matrix_2 = growth * Matrix2::rotation(-angle);

            // Clean up before new tree drawing
            self.branches = vec![Branch::new_stem()];
            self.visualizer.clear(None);
        }
    }
}