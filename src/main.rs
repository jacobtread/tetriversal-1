extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use serde::{Deserialize, Serialize};
use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::Message;
use futures::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use log::{info, error, debug, Level};
use colored::*;
use std::str::FromStr;

#[derive(Deserialize, Serialize)]
struct Packet {
    pub id: i32,
    pub name: Option<String>,
    pub uuid: Option<String>,
    pub reason: Option<String>,
    pub key: Option<String>,
    pub lines: Option<Vec<String>>,
    pub tile: Option<Vec<Vec<i32>>>,
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl Packet {
    pub fn join_packet(name: String) -> Packet {
        Self {
            id: 1,
            name: Some(name),
            uuid: None,
            reason: None,
            key: None,
            lines: None,
            tile: None,
            x: None,
            y: None,
        }
    }
}


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
            }
            Tile::Empty => {
                [0.0, 0.0, 0.0, 0.0]
            }
        }
    }
}

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

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::LogBuilder::new().format(|record| format!("[{}]", match Level::from_str(&record.level().to_string()).unwrap() {
        Level::Error => record.level().to_string().as_str().red().bold(),
        Level::Warn => record.level().to_string().as_str().yellow().bold(),
        Level::Info => record.level().to_string().as_str().blue().bold(),
        Level::Debug => record.level().to_string().as_str().bright_white().bold(),
        Level::Trace => record.level().to_string().as_str().white().bold(),
    })).init().unwrap();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let outgoing_queue: Arc<Mutex<VecDeque<Message>>> = Arc::new(Mutex::new(VecDeque::new()));

    tokio::spawn(async move {
        info!("Connecting to websocket");

        let (mut stream, _) = match connect_async("ws://echo.websocket.org").await {
            Ok(val) => { val }
            Err(e) => {
                error!("Error connecting to WebSocket\n{}", e);
                return;
            }
        };

        info!("Connected to websocket");

        stream.send(Message::Text("Hentai".to_string()));
        let msg = stream.next().await.ok_or("Didn't receive anything").unwrap().unwrap();
        debug!("msg: {}", msg);
    });

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: (2..22).map(|a| (1..13).map(|b| match (a * b) % 4 {
            0 => Tile::Empty,
            1 => Tile::Blue,
            2 => Tile::Green,
            3 => Tile::Red,
            _ => Tile::Empty
        }).collect()).collect(),
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

