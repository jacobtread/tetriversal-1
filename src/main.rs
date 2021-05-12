extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::Message;
use colored::*;
use futures::prelude::*;
use glutin_window::GlutinWindow as Window;
use log::{debug, error, info};
use opengl_graphics::{GlGraphics, OpenGL};
use packets::outgoing::OutgoingPackets;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use state::App;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

pub mod packets;
pub mod state;
pub mod util;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}]: {}",
                match record.level() {
                    log::Level::Error => "  ERROR  ".red().bold(),
                    log::Level::Warn => " WARNING ".yellow().bold(),
                    log::Level::Info => "  INFO   ".green().bold(),
                    log::Level::Debug => "  DEBUG  ".blue().bold(),
                    log::Level::Trace => "  TRACE  ".white().bold(),
                },
                record.args()
            )
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

    let outgoing_queue: Arc<Mutex<VecDeque<OutgoingPackets>>> = Arc::new(Mutex::new(VecDeque::new()));

    tokio::spawn(async move {
        info!("Connecting to websocket");

        let (mut stream, _) = match connect_async("ws://echo.websocket.org").await {
            Ok(val) => val,
            Err(e) => {
                error!("Error connecting to WebSocket\n{}", e);
                return;
            }
        };

        info!("Connected to websocket");

        stream.send(Message::Text("Hentai".to_string())).await.unwrap();
        let msg = stream
            .next()
            .await
            .ok_or("Didn't receive anything")
            .unwrap()
            .unwrap();
        debug!("msg: {}", msg);
    });

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: (2..22)
            .map(|a| {
                (1..13)
                    .map(|b| match (a * b) % 4 {
                        0 => util::Tile::Empty,
                        1 => util::Tile::Blue,
                        2 => util::Tile::Green,
                        3 => util::Tile::Red,
                        _ => util::Tile::Empty,
                    })
                    .collect()
            })
            .collect(),
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
