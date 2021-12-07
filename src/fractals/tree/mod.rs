use super::super::visuals::{
    visualizer::{WIDTH}
};

use super::super::math::{
    matrix::Matrix2,
    vector::Vector2
};

use std::f32::consts::PI;

// consts
const DEPTH : usize = 8;
const START_POINT : Vector2 = Vector2 {x: WIDTH / 2.0, y: 550.0};
const END_POINT: Vector2 = Vector2 {x: START_POINT.x, y: 400.0};
const ANGLE: f32 = PI / 6.0;
const GROWTH_FACTOR : f32 = 0.7;

/// # `Branch`
/// Structure for the building blocks of the Fractal Trees
pub struct Branch {
    start: Vector2,
    end: Vector2
}

impl Branch {
    /// # `new`
    /// Creates a new branch by taking a starting and ending point `Vector2`
    pub fn new(start: Vector2, end: Vector2) -> Branch {
        Branch {
            start: start,
            end: end
        }
    }

    /// # `new_stem`
    /// Creates a new branch with using the initial starting and ending points
    pub fn new_stem() -> Branch {
        Branch {
            start: START_POINT,
            end: END_POINT
        }
    }

    /// # `grow`
    /// Creates the next branch by taking the growth matrix `Matrix2`
    pub fn grow(&self, growth_matrix: Matrix2) -> Branch {
        let next_branch = self.end - self.start;

        Branch {
            start: self.end,
            end: self.end + growth_matrix * next_branch,
        }
    }
}

// === SUB MODS === 

pub mod diablos_tree;
pub mod simple;