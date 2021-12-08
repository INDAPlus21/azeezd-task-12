use crate::visuals::{
    visualizer::{WIDTH, HEIGHT, Visualizer},
    colours
};
use minifb::{MouseButton, MouseMode};

use std::f32::consts::PI;

use crate::math::{
    matrix::Matrix2,
    vector::Vector2,
    lsystem::LSystem
};

use super::Branch;

const ANGLE : f32 = PI / 7.2;
const GROWTH : f32 = 0.6;
const STARTING_LENGTH : f32 = 20.0;

/// # `LSystemTree`
/// Structure that generates trees based on the L-System
pub struct LSystemTree {
    visualizer: Visualizer,
    branches: Vec<Branch>,
    rules: LSystem,
    length: f32
}

impl LSystemTree {
    /// # `new`
    /// Creates a new L-System tree fractal
    pub fn new() -> LSystemTree {
        let mut lsystem = LSystem::new();
        lsystem.add_rule('F', "-F+F+[+F+F-FF]-[-FF-F+F]".to_string());
        LSystemTree {
            visualizer: Visualizer::new(Some(100)),
            branches: vec![Branch::new_stem()],
            rules: lsystem,
            length: STARTING_LENGTH
        }
    }

    /// # `generate`
    /// Generates the next set of branches based on the given sentence
    fn generate(&mut self, sentence: &String) {
        let mut original_matrix = Matrix2::identity(); // Matrix of the whole tree
        let mut current_branch = self.branches[0]; // Branch currently growing
        let mut current_matrix = Matrix2::identity(); // Matrix of the current sub-branch

        let mut previous_branches : Vec<Branch> = Vec::new(); // Storing previous main/sub branches
        let mut previous_matrices : Vec<Matrix2> = Vec::new(); // Storing previous matrices of previous sub branches

        // Rotation matrices
        let rot_pos = Matrix2::rotation(ANGLE);
        let rot_neg = rot_pos.inverse().unwrap();

        for character in sentence.chars() {
            match character {
                'F' => { // Move forward
                    let new_branch = Branch::new(current_branch.end, (original_matrix * Vector2::new(0.0, -self.length)) + current_branch.end);
                    self.branches.push(new_branch);
                    current_branch = new_branch;
                },

                '+' => { // Rotate positively
                    current_matrix = rot_pos * current_matrix;
                    original_matrix = rot_pos * original_matrix;
                },

                '-' => { // Rotate negatively
                    current_matrix = rot_neg * current_matrix;
                    original_matrix = rot_neg * original_matrix;
                },

                '[' => { // Start a sub branch, store previous data
                    previous_branches.push(current_branch);
                    previous_matrices.push(current_matrix);
                    current_matrix = Matrix2::identity();
                },

                ']' => { // Exit sub branch, pop previous data
                    original_matrix = current_matrix.inverse().unwrap() * original_matrix;

                    current_branch = previous_branches.pop().unwrap();
                    current_matrix = previous_matrices.pop().unwrap();
                },
                _ => {}
            }
        }

        self.length *= GROWTH; // reduce length each generation
    } 

    pub fn draw(&mut self) {
        let mut sentence : String = "F".to_string();

        // Draw loop, no need for visualizer.end()
        while self.visualizer.window.is_open() {
            // Draw current branches
            for branch in self.branches.iter() {
                self.visualizer.draw_line(branch.start, branch.end, colours::WHITE, 1);
            }
            self.visualizer.apply_buffer();

            // Clear tree
            self.branches = vec![Branch::new_stem()];

            // Upon user mouse click this exits the loop and continues down
            while !self.visualizer.window.get_mouse_down(MouseButton::Left) && self.visualizer.window.is_open() {
                self.visualizer.window.update();
            }

            // Generate new tree
            sentence = self.rules.generate(sentence);
            self.generate(&sentence);

            // Clean up before new tree drawing
            self.visualizer.clear(None);
        }
    }
}