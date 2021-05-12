use crate::util::Tile;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use futures::channel::mpsc::Sender;
use crate::packets::outgoing::OutgoingPacket;
use crate::util;
use std::collections::VecDeque;
use crate::packets::incoming::IncomingPacket;
use futures_util::lock::Mutex;
use std::sync::Arc;
use log::{debug};
use futures_util::SinkExt;

#[allow(dead_code)]
pub struct App {
    pub gl: GlGraphics,
    // OpenGL drawing backend.
    pub grid: Vec<Vec<Tile>>,
    outgoing_stream: Sender<OutgoingPacket>,
    incoming_queue: Arc<Mutex<VecDeque<IncomingPacket>>>
}

impl App {
    pub async fn new(gl: GlGraphics, mut outgoing_stream: Sender<OutgoingPacket>, incoming_queue: Arc<Mutex<VecDeque<IncomingPacket>>>) -> Self {
        outgoing_stream.send(OutgoingPacket::JoinRequestPacket {name: "Yes".to_string()}).await.unwrap();
        Self {
            gl,
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
            outgoing_stream,
            incoming_queue
        }
    }
    pub fn render(&mut self, args: &RenderArgs) {
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

    pub async fn update(&mut self, _args: &UpdateArgs) {
        // Get the message queue
        let mut msg_queue = self.incoming_queue.lock().await;
        let message_queue = msg_queue.clone();

        // Empty the message queue
        msg_queue.clear();
        drop(msg_queue);

        // Handle all of the messages
        for a in message_queue.iter() {
            debug!("{:?}", a);
        }
    }
}
