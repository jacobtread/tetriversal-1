extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use log::{debug, error, info};
use opengl_graphics::{GlGraphics, OpenGL};
use packets::outgoing::OutgoingPackets;
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use state::App;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::util::logging::init_log;
use piston::{RenderEvent, UpdateEvent};
use crate::socket::create_socket;
use futures::channel::mpsc::channel;
use futures::{SinkExt, StreamExt};

pub mod packets;
pub mod state;
pub mod util;
pub mod socket;

#[tokio::main]
async fn main() {
    init_log();
    let async_runtime = tokio::runtime::Runtime::new().unwrap();
    let (mut out_tx, out_rx) = channel(1024);
    let (in_tx, in_rx) = channel(1024);
    let aws = async_runtime.spawn(create_socket(out_rx, in_tx));

    out_tx.send(OutgoingPackets::JoinRequestPacket {
        name: "no".to_string()
    }).await;

    // in_rx.for_each(|e| async move {
    //     info!("{}", e);
    // }).await;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let outgoing_queue: Arc<Mutex<VecDeque<OutgoingPackets>>> = Arc::new(Mutex::new(VecDeque::new()));


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
