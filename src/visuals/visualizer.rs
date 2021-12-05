extern crate minifb;
use minifb::{Window, WindowOptions, Key};

use crate::math::vector::*;

// Screen size
pub const WIDTH : f32 = 800.0;
pub const HEIGHT : f32 = 600.0;

// As usize for buffer access
const U_WIDTH : usize = WIDTH as usize;
const U_HEIGHT : usize = HEIGHT as usize;

/// # `Visualizer`
/// Struct used to manipulate the pixel buffer of the screen
pub struct Visualizer {
    pub buffer: Vec<u32>,
    pub window: Window
}

impl Visualizer {
    /// # `new`
    /// Creates a new Visualizer with an empty buffer and returns it.
    /// ## Parameters
    /// `update_rate: Option<u64>` - Amount of milliseconds between each screen update. `None` is for default which is 17ms
    pub fn new(update_rate: Option<u64>) -> Visualizer {
        let mut vis = Visualizer {
            buffer: vec![0; WIDTH as usize * HEIGHT as usize], 
            window: Window::new("Fractal Visualizer", 
                                WIDTH as usize, 
                                HEIGHT as usize, 
                                WindowOptions::default()).unwrap()
        };

        vis.window.limit_update_rate(
            match update_rate {
                Some(t) => Some(std::time::Duration::from_millis(t)),
                _ => Some(std::time::Duration::from_millis(17))
            }
        );

        vis
    }

    /// # `coord_to_buffer_idx`
    /// Takes a coordinate in the form of `(usize, usize)` and returns the index of that point in the buffer.
    /// Returns a result which will contain and error if the coordinate are out of bounds
    fn coord_to_buffer_idx(coordinate: Vector2) -> Result<usize, &'static str> {
        match coordinate {
            _ if coordinate.x as isize >= WIDTH as isize => Err("Index out of bounds!"),
            _ if coordinate.y as isize >= HEIGHT as isize => Err("Index out of bounds!"),
            _ => Ok(coordinate.x as usize + coordinate.y as usize * U_WIDTH)
        }
    }

    /// # `set_pixel`
    /// Takes a coordinate value as `(usize, usize)` and a colour value as `u32` and sets the pixel at that coordinate to that given colour.
    /// Returns a result which will contain an error if the coordinate is out of bounds
    pub fn set_pixel(&mut self, coordinate: Vector2, value: u32) -> Result<(), &'static str> {
        match Visualizer::coord_to_buffer_idx(coordinate) {
            Ok(idx) => {
                if let Some(e) = self.buffer.get_mut(idx) {
                    *e = value;
                }
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    /// # `draw_line_at`
    /// PART OF MAIN DRAW LINE FUNCTION
    /// Draws a line using the Bersenham Line Algorithm byt taking a starting point `(usize, usize)`, terminal point `(usize, usize)` and colour data `u32`
    fn draw_line_at(&mut self, start: Vector2, end: Vector2, colour: u32) -> Result<(), &'static str> {
        let mut start = (start.x as isize, start.y as isize);
        let end = (end.x as isize, end.y as isize);

        let dx = (end.0 - start.0).abs();
        let sx = if start.0 < end.0 { 1 } else {-1 };
        let dy = -(end.1 - start.1).abs();
        let sy = if start.1 < end.1 { 1 } else {-1 };
        let mut error = dx + dy;

        loop {
            self.set_pixel(Vector2::from_isize(start), colour);
            if start.0 == end.0 && start.1 == end.1 { break;}
            let e2 = 2 * error;
            if e2 >= dy {
                error += dy;
                start.0 += sx;
            }
            if e2 <= dx {
                error += dx;
                start.1 += sy;
            }
        }

        Ok(())
    }


    /// # `draw_line`
    /// Takes a starting position `(usize)` and an terminal position `(usize, usize)` and a colour `(u32)` and thickness `usize`
    /// then draws a line from a starting point to an terminal point using Bersenham Line Algorithm
    pub fn draw_line(&mut self, start: Vector2, end: Vector2, colour: u32, thickness: usize) -> Result<(), &'static str> {
        for thick in 0..thickness {
            let thick_coord = Vector2::new(thick as f32, thick as f32);
            self.draw_line_at(
                start - thick_coord, 
                end - thick_coord,
                colour);
        }

        Ok(())
    }

    /// # `draw_circle_edge`
    /// PART OF ALGORITHM!
    /// Draws a part of the circle's edge by taking the center of the circle in the form `(usize, usize)` 
    /// and the part of the edges to draw `(usize, usize)`. 
    fn draw_circle_edge(&mut self, center: Vector2, coord: Vector2, colour: u32) {

        self.set_pixel(Vector2::new(center.x + coord.x, center.y + coord.y), colour);
        self.set_pixel(Vector2::new(center.x - coord.x, center.y + coord.y), colour);
        self.set_pixel(Vector2::new(center.x + coord.x, center.y - coord.y), colour);
        self.set_pixel(Vector2::new(center.x - coord.x, center.y - coord.y), colour);
        
        self.set_pixel(Vector2::new(center.x + coord.y, center.y + coord.x), colour);
        self.set_pixel(Vector2::new(center.x - coord.y, center.y + coord.x), colour);
        self.set_pixel(Vector2::new(center.x + coord.y, center.y - coord.x), colour);
        self.set_pixel(Vector2::new(center.x - coord.y, center.y - coord.x), colour);
    }

    /// # `draw_circle`
    /// Takes a center `(usize, usize)` and a radius `(usize, usize)` and a colour `(u32)` 
    /// and draws a circle using the Bersenham Circle Algorithm
    pub fn draw_circle(&mut self, center: Vector2, radius: f32, colour: u32) {
        let mut x = 0.0;
        let mut y = radius;
        let mut d = 3.0 - 2.0 * radius;

        self.draw_circle_edge(center, Vector2::new(x, y), colour);

        while y >= x {
            x += 1.0;
            if d > 0.0 {
                y -= 1.0;
                d = d + 4.0 * (x - y) + 10.0;
            }
            else {
                d = d + 4.0 * x + 6.0;
            }
            self.draw_circle_edge(center, Vector2::new(x, y), colour);
        }
    }

    /// # `apply_buffer`
    /// Draws the content of the buffer unto the window
    pub fn apply_buffer(&mut self) {
        self.window.update_with_buffer(&self.buffer, U_WIDTH, U_HEIGHT);
    }

    /// # `end`
    /// Should always be placed at the end of a visualization to avoid window from closing
    pub fn end(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update();
        }
    }

    /// # `clear`
    /// Fills the enter buffer with the given colour. `None` is default for black
    pub fn clear(&mut self, colour: Option<u32>) {
        let colour = match colour {
            Some(a) => a,
            _ => super::colours::BLACK
        };
        self.buffer = vec![colour; U_WIDTH * U_HEIGHT];
    }
}