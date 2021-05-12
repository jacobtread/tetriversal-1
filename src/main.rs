extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use log::{info};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use state::App;
use std::collections::VecDeque;
use std::sync::Arc;
use futures_util::lock::Mutex;
use crate::util::logging::init_log;
use piston::{RenderEvent, UpdateEvent};
use crate::socket::create_socket;
use futures::channel::mpsc::channel;
use crate::packets::incoming::IncomingPacket;

pub mod packets;
pub mod state;
pub mod util;
pub mod socket;

#[tokio::main]
async fn main() {
    // Initialize the logging system
    init_log();

    info!("Launching game");

    // Create streams to send packets to server
    let (out_tx, out_rx) = channel(1024);

    // Queue of packets coming from server
    let input_queue: Arc<Mutex<VecDeque<IncomingPacket>>> = Arc::new(Mutex::new(VecDeque::new()));

    // Setup the websocket
    let in_q = input_queue.clone();
    tokio::spawn(create_socket(out_rx, in_q));

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl), out_tx, input_queue).await;

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args).await;
        }
    }
}
