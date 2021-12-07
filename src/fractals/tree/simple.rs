use crate::visuals::{
    visualizer::Visualizer,
    colours
};

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
            visualizer: Visualizer::new(Some(500)),
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
        for curr_depth in 0..super::DEPTH {
            for branch in self.branches.iter() {
                self.visualizer.draw_line(branch.start, branch.end, colours::CERISE, super::DEPTH - curr_depth);
            }
            self.visualizer.apply_buffer();

            self.generate();
        }

        self.visualizer.end();
    }
}