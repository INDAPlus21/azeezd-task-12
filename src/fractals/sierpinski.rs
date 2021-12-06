use super::super::visuals::{
    visualizer::Visualizer,
    visualizer::{WIDTH},
    colours
};

use super::super::math::{
    matrix::Matrix2,
    vector::Vector2
};

use std::f32::consts::PI;

// Consts
const DEPTH : usize = 10;
const POINT_A : Vector2 = Vector2 {x: 100.0, y: 550.0};
const POINT_B : Vector2 = Vector2 {x: WIDTH - 100.0, y: 550.0};
const SQRT_3 : f32 = 1.73205080757;

/// # `Triangle`
/// A good ol' triangle
struct Triangle {
    a: Vector2,
    b: Vector2,
    c: Vector2
}

impl Triangle {
    /// # `draw`
    /// Draws the edges of the triangle by taking a `Visualizer`
    pub fn draw(&self, visualizer: &mut Visualizer) {
        visualizer.draw_line(self.a, self.b, colours::CERISE, 1);
        visualizer.draw_line(self.a, self.c, colours::CERISE, 1);
        visualizer.draw_line(self.b, self.c, colours::CERISE, 1);
    }

    // === GET POINTS BETWEEN EACH EDGE OF THE TRIANGLE ===

    pub fn point_ab(&self) -> Vector2 {
        0.5 * (self.b - self.a) + self.a
    }

    pub fn point_ac(&self) -> Vector2 {
        0.5 * (self.c - self.a) + self.a
    }

    pub fn point_bc(&self) -> Vector2 {
        0.5 * (self.c - self.b) + self.b
    }
}

/// # `SierpinskiTriangle`
/// Structure that draws the Sierpinski Triangle Fractal
pub struct SierpinskiTriangle {
    visualizer: Visualizer,
    triangles: Vec<Triangle>
}

impl SierpinskiTriangle {

    /// # `new`
    /// Initializes the Sierpinski Triangle fractal
    pub fn new() -> SierpinskiTriangle {
        // Generate the top point of the equilateral by finding half the base and rotating it by 90 degrees then scaling it appropiately (sqrt of 3)
        let point_c = 0.5 * (POINT_B - POINT_A);
        let rot_matrix = SQRT_3 * Matrix2::rotation(-PI / 2.0);
        let point_c = rot_matrix * point_c + (POINT_A + point_c);

        SierpinskiTriangle {
            visualizer: Visualizer::new(Some(500)),
            triangles: vec![Triangle{a: POINT_A, b: POINT_B, c: point_c}] // Initial triangle
        }
    }

    /// # `draw`
    /// Draw the fractal on the screen
    pub fn draw(&mut self) {
        for _ in 0..DEPTH {
            self.generate();
            for elem in self.triangles.iter() {
                elem.draw(&mut self.visualizer);
            }
            self.visualizer.apply_buffer();
        }

        self.visualizer.end();
    }

    /// # `generate`
    /// Generates the next set of triangles of the Sierpinski Triangle
    pub fn generate(&mut self) {
        let mut next : Vec<Triangle> = Vec::with_capacity(self.triangles.len() * 3); // Each iteration, every triangle is split into 3

        for triangle in self.triangles.iter() {
            let ab = triangle.point_ab();
            let ac = triangle.point_ac();
            let bc = triangle.point_bc();

            // Push new split triangles
            next.push(Triangle {a: triangle.a, b: ab, c: ac});
            next.push(Triangle {a: ab, b: triangle.b, c: bc});
            next.push(Triangle {a: ac, b: bc, c: triangle.c});
        }

        self.triangles = next;
    }
}