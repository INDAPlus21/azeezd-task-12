extern crate minifb;
use minifb::{Window, WindowOptions};

// Screen size
const WIDTH : usize = 800;
const HEIGHT : usize = 600;

/// # `Visualizer`
/// Struct used to manipulate the pixel buffer of the screen
pub struct Visualizer {
    pub buffer: Vec<u32>,
    pub window: Window
}

impl Visualizer {
    /// # `new`
    /// Creates a new Visualizer with an empty buffer and returns it
    pub fn new() -> Visualizer {
        let mut vis = Visualizer {
            buffer: vec![0; WIDTH * HEIGHT], 
            window: Window::new("Fractal Visualizer", 
                                WIDTH, 
                                HEIGHT, 
                                WindowOptions::default()).unwrap()
        };

        vis.window.limit_update_rate(Some(std::time::Duration::from_millis(17)));

        vis
    }

    /// # `coord_to_buffer_idx`
    /// Takes a coordinate in the form of `(usize, usize)` and returns the index of that point in the buffer.
    /// Returns a result which will contain and error if the coordinate are out of bounds
    fn coord_to_buffer_idx(coordinate: (usize, usize)) -> Result<usize, &'static str> {
        match coordinate {
            _ if coordinate.0 >= WIDTH => Err("Index out of bounds!"),
            _ if coordinate.1 >= HEIGHT => Err("Index out of bounds!"),
            _ => Ok(coordinate.0 + coordinate.1 * WIDTH)
        }
    }

    /// # `set_pixel`
    /// Takes a coordinate value as `(usize, usize)` and a colour value as `u32` and sets the pixel at that coordinate to that given colour.
    /// Returns a result which will contain an error if the coordinate is out of bounds
    pub fn set_pixel(&mut self, coordinate: (usize, usize), value: u32) -> Result<(), &'static str> {
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

    /// # `draw_line`
    /// Takes a starting position `(usize)` and an ending position `(usize, usize)` and a colour `(u32)` 
    /// then draws a line from a starting point to an ending point using Bersenham Line Algorithm
    pub fn draw_line(&mut self, start: (usize, usize), end: (usize, usize), colour: u32) -> Result<(), &'static str> {
        let start = (start.0 as isize, start.1 as isize);
        let end = (end.0 as isize, end.1 as isize);

        let dx = end.0 - start.0;
        let dy = end.1 - start.1;
        let mut y = start.1;
        let mut error = 0;

        let mut x = start.0;

        while x <= end.0 {
            match self.set_pixel((x as usize, y as usize), colour) {
                Ok(_) => {
                    error += dy;
                    if (error << 1) >= dx {
                        y += 1;
                        error -= dx;
                    }
                    x += 1;
                }
                Err(err) => return Err(err)
            }
        }

        Ok(())
    }

    /// # `draw_circle_edge`
    /// PART OF ALGORITHM!
    /// Draws a part of the circle's edge by taking the center of the circle in the form `(usize, usize)` 
    /// and the part of the edges to draw `(usize, usize)`. 
    fn draw_circle_edge(&mut self, center: (usize, usize), coord: (usize, usize), colour: u32) {
        let center = (center.0 as isize, center.1 as isize);
        let coord = (coord.0 as isize, coord.1 as isize);

        self.set_pixel(((center.0 + coord.0) as usize, (center.1 + coord.1) as usize), colour);
        self.set_pixel(((center.0 - coord.0) as usize, (center.1 + coord.1) as usize), colour);
        self.set_pixel(((center.0 + coord.0) as usize, (center.1 - coord.1) as usize), colour);
        self.set_pixel(((center.0 - coord.0) as usize, (center.1 - coord.1) as usize), colour);
        self.set_pixel(((center.0 + coord.1) as usize, (center.1 + coord.0) as usize), colour);
        self.set_pixel(((center.0 - coord.1) as usize, (center.1 + coord.0) as usize), colour);
        self.set_pixel(((center.0 + coord.1) as usize, (center.1 - coord.0) as usize), colour);
        self.set_pixel(((center.0 - coord.1) as usize, (center.1 - coord.0) as usize), colour);
    }

    /// # `draw_circle`
    /// Takes a center `(usize, usize)` and a radius `(usize, usize)` and a colour `(u32)` 
    /// and draws a circle using the Bersenham Circle Algorithm by
    pub fn draw_circle(&mut self, center: (usize, usize), radius: usize, colour: u32) {
        let mut x = 0 as isize;
        let mut y = radius as isize;
        let mut d = 3 - 2 * radius as isize;

        self.draw_circle_edge(center, (x as usize,y as usize), colour);

        while y >= x {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            }
            else {
                d = d + 4 * x + 6;
            }
            self.draw_circle_edge(center, (x as usize,y as usize), colour);
        }
    }
}