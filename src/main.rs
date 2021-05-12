extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::Message;
use futures::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use log::{info, error, debug};
use std::io::prelude::*;
use colored::*;

pub mod packets;
pub mod state;
pub mod util;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::builder()
    .format(|buf, record| {
        writeln!(buf, "[{}]: {}", match record.level() {
    log::Level::Error => " ERROR ".red().bold(),
    log::Level::Warn => "WARNING".yellow().bold(),
    log::Level::Info => "  INFO ".green().bold(),
    log::Level::Debug => " DEBUG ".blue().bold(),
    log::Level::Trace => " TRACE ".white().bold()
}, record.args())
    })
    .init();

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

