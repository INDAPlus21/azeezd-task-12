use crate::visuals::{
    visualizer::{Visualizer, U_WIDTH, U_HEIGHT, HEIGHT, WIDTH},
    colours};
use crate::math::vector::Vector2;

// Consts
const MAX_ITERATION : usize = 300;
const ZOOM_FACTOR : f32 = 0.95;
const ZOOM_START : f32 = 2.0;
const SCALE_BASIS : Vector2 = Vector2 {x: 2.0, y: 1.5}; // Best if same ratio as the screen (4:3)
const ZOOM_POINT : Vector2 = Vector2{x: -1.139083E-1, y: 8.990149E-1};

/// # `Mandelbrot`
/// Structures that visualizes (with a lot of lags) the Mandelbrot set 
pub struct Mandelbrot {
    visualizer: Visualizer,
    origin: Vector2,
    scale_basis: Vector2
}

impl Mandelbrot {
    /// # `new`
    /// Initializes the Mandelbrot visualizer
    pub fn new() -> Mandelbrot {
        Mandelbrot {
            visualizer: Visualizer::new(None),
            origin: ZOOM_POINT,
            scale_basis: SCALE_BASIS
        }
    }

    /// # `draw`
    /// Draws the fractal on the screen
    pub fn draw(&mut self) {
        let mut zoom = ZOOM_START;
        
        while self.visualizer.window.is_open() {
            for px in 0..U_WIDTH {
                for py in 0..U_HEIGHT {
                    // The Mandelbrot set is drawn using the Complex function f(z) = z^2 + C, fed recursively 
                    // and for the different values of C tests if the value of the recursion blows up.
                    // A simple test is to see if after some iterations the complex modulous (norm, length or absolute value) is larger than 2
                    // Based on the number of iterations a different colour (or hue) is drawn on the screen in that corresponding pixel position 
                    
                    // Get the +C, based on the pixel on the window after scaling and translating appropriately
                    let coordinate = Vector2::new(((px as f32) / WIDTH - 0.5) * self.scale_basis.x + self.origin.x, 
                                                  ((py as f32) / HEIGHT - 0.5) * self.scale_basis.y + self.origin.y);

                    // Taking the "scalar" of the imaginary and real part 
                    // (ignoring the complex hassle because we only need length and that can be computed without complex computations)
                    let mut x = 0.0;
                    let mut y = 0.0;
                    let mut iteration = 0; // amount of iterations
                    
                    // Looping while the norm (x^2 + y^2) is less than 2^2 and while we are less than the upper limit of iterations
                    while x * x + y * y <= 4.0 && iteration < MAX_ITERATION {

                        // if we say z = (x+yi) then f(z) = x^2 - y^2 + 2xyi + c
                        // So x^2 - y^2 + c's real part is the new real part and
                        // 2xy + c's imaginary part is the new imaginary part's coefficient
                        let temp = x * x - y * y + coordinate.x;
                        y = 2.0 * x * y + coordinate.y;
                        x = temp;
                        iteration += 1;
                    }
                    self.visualizer.set_pixel(Vector2::from_isize((px as isize, py as isize)), Mandelbrot::get_colour(iteration));
                }
            }
            self.visualizer.apply_buffer();

            // Change zoom and apply it to scale basis
            zoom *= ZOOM_FACTOR;
            self.scale_basis = SCALE_BASIS * zoom;
        }
    }

    /// # `get_colour`
    /// Gets the colour of the pixel to draw based on the amount of iterations
    fn get_colour(iterations: usize) -> u32 {
        (colours::BLACK + iterations as u32) << 10
    }
}