extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;


#[derive(Clone, Debug)]
enum Tile {
    Red,
    Green,
    Blue,
    Empty,
}

impl Tile {
    fn get_color(&self) -> [f32; 4] {
        match self {
            Tile::Red => {
                [1.0, 0.0, 0.0, 1.0]
            }
            Tile::Green => {
                [0.0, 1.0, 0.0, 1.0]
            }
            Tile::Blue => {
                [0.0, 0.0, 1.0, 1.0]
            },
            Tile::Empty => {
                [0.0, 0.0, 0.0, 0.0]
            }
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    grid: Vec<Vec<Tile>>
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
                        .trans(x_offset as f64 * 26.0, y_offset as f64* 26.0);

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

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: (2..22).map(|a| (1..13).map(|b| match (a * b) % 4 {
            0 => Tile::Empty,
            1 => Tile::Blue,
            2 => Tile::Green,
            3 => Tile::Red,
            _ => Tile::Empty
        }).collect()).collect()
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

