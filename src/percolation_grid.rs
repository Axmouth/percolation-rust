extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use rand::*;

use super::constants::BLOCK_SIZE;
use super::percolation::*;

pub struct PercolationGrid {
    gl: GlGraphics, // OpenGL drawing backend.
    percolation: Percolation,
    rows: u32,
    cols: u32,
    block_size: u32,
}

impl PercolationGrid {
    pub fn new(opengl: OpenGL, rows: u32, cols: u32) -> Self {
        let grid = PercolationGrid {
            gl: GlGraphics::new(opengl),
            percolation: Percolation::new(rows, cols),
            block_size: BLOCK_SIZE,
            rows,
            cols,
        };

        return grid;
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const LIGHT_BLUE: [f32; 4] = [0.678, 0.847, 0.9019, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        for row in 0..self.percolation.get_rows() {
            for col in 0..self.percolation.get_cols() {
                if self.percolation.is_full(row + 1, col + 1) {
                    let square = graphics::rectangle::square(
                        (col as u32 * self.block_size) as f64,
                        (row as u32 * self.block_size) as f64,
                        20_f64,
                    );
                    self.gl.draw(args.viewport(), |c, gl| {
                        let transform = c.transform;
                        graphics::rectangle(LIGHT_BLUE, square, transform, gl);
                    });
                    continue;
                }
                if self.percolation.is_open(row + 1, col + 1) {
                    let square = graphics::rectangle::square(
                        (col as u32 * self.block_size) as f64,
                        (row as u32 * self.block_size) as f64,
                        20_f64,
                    );
                    self.gl.draw(args.viewport(), |c, gl| {
                        let transform = c.transform;
                        graphics::rectangle(WHITE, square, transform, gl);
                    });
                }
            }
        }
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        if !self.percolation.percolates() {
            self.open_random_site();
        }
    }

    pub fn pressed(&mut self, btn: &Button) {}

    pub fn reset_grid_state(&mut self) {
        self.percolation = Percolation::new(self.rows, self.cols);
    }

    fn convert_indice(&self, row: u32, col: u32) -> u32 {
        return row * self.cols + col;
    }

    pub fn open_random_site(&mut self) {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(1.0, (self.rows + 1) as f64) as u32;
        let col = rng.gen_range(1.0, (self.cols + 1) as f64) as u32;
        self.percolation.open(row, col);
    }
}
