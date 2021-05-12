use crate::util::Tile;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    grid: Vec<Vec<Tile>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 25.0);
        let (x, y) = (10.0, 10.0);
        let grid = self.grid.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([1.0, 1.0, 1.0, 1.0], gl);

            for (y_offset, row) in grid.iter().enumerate() {
                for (x_offset, tile) in row.iter().enumerate() {
                    let transform = c
                        .transform
                        .trans(x, y)
                        .trans(x_offset as f64 * 26.0, y_offset as f64 * 26.0);

                    // Draw a box rotating around the middle of the screen.
                    rectangle(tile.get_color(), square, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }
}
