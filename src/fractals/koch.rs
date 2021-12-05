use super::super::visuals::{
    visualizer::Visualizer,
    visualizer::{WIDTH},
    colours
};

use super::super::math::{
    matrix::Matrix2,
    vector::Vector2
};

// Consts
const DEPTH : usize = 10;
const ANGLE : f32 = -std::f32::consts::PI / 3.0;
const INIT_LINE_START : Vector2 = Vector2 {x: 50.0, y:300.0};
const INIT_LINE_END : Vector2 = Vector2 {x: WIDTH - INIT_LINE_START.x , y: INIT_LINE_START.y};

struct KochLine {
    start: Vector2,
    end: Vector2
}

impl KochLine {
    
    /// # `draw`
    /// Draws the line on the screen
    pub fn draw(&self, visualizer: &mut Visualizer) {
        visualizer.draw_line(self.start, self.end, colours::RUST, 1);
    }

    // === GET POINTS OF THE DIVIDED KOCH LINES ===

    pub fn koch_a(&self) -> Vector2 {
        self.start
    }

    pub fn koch_b(&self) -> Vector2 {
        let vector = self.end - self.start;
        self.start + vector / 3.0
    }

    pub fn koch_c(&self) -> Vector2 {
        let rot_matrix = Matrix2::rotation(ANGLE);
        let vector = (self.end - self.start) / 3.0;
        let vector_rotated = rot_matrix * vector; 

        vector + vector_rotated + self.start
    }

    pub fn koch_d(&self) -> Vector2 {
        let vector = self.end - self.start;
        self.start + vector * (2.0 / 3.0)
    }

    pub fn koch_e(&self) -> Vector2 {
        self.end
    }
}

/// # `Koch`
/// The Koch Fractal visualizer
pub struct Koch {
    koch_lines: Vec<KochLine>,
    visualizer: Visualizer
}

impl Koch {

    /// # `new`
    /// Initializes a new Koch Line fractal
    pub fn new() -> Koch {
        let lines : Vec<KochLine> = vec![
            KochLine{start: INIT_LINE_START, end: INIT_LINE_END},
            KochLine{start: INIT_LINE_END, end: INIT_LINE_START},
            ];
        Koch {
            koch_lines: lines,
            visualizer: Visualizer::new(Some(300)) // 300ms frame rate
        }
    }

    /// # `generate`
    /// Generates the next set of lines of the fractal
    fn generate(&mut self) {
        let mut next : Vec<KochLine> = Vec::new();

        for line in self.koch_lines.iter() {
            let a = line.koch_a();
            let b = line.koch_b();
            let c = line.koch_c();
            let d = line.koch_d();
            let e = line.koch_e();

            // a - b
            next.push(KochLine{
                start: a,
                end: b});

            // b - c
            next.push(KochLine{
                start: b,
                end: c});

            // c - d
            next.push(KochLine{
                start: c,
                end: d});

            // d - e
            next.push(KochLine{
                start: d,
                end: e});
        }

        self.koch_lines = next;
    }

    /// # `draw`
    /// Draws the currently stored lines of the fractal
    pub fn draw(&mut self) {
        for _ in 0..DEPTH { 
            for line in self.koch_lines.iter() {
                line.draw(&mut self.visualizer);
            }
            self.visualizer.apply_buffer();
            self.visualizer.clear(None);
            self.generate();
        }

        self.visualizer.end()
    }
}