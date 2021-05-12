use log::{info, debug};
use url::Url;
use tokio_tungstenite::tungstenite::Message;
use futures::channel::mpsc::{Receiver};
use crate::packets::outgoing::OutgoingPacket;
use crate::packets::incoming::IncomingPacket;
use serde::Serialize;
use futures_util::{future, pin_mut, StreamExt};
use std::collections::VecDeque;
use futures::lock::Mutex;
use std::sync::Arc;

pub async fn create_socket(outbound_stream: Receiver<OutgoingPacket>, input_queue: Arc<Mutex<VecDeque<IncomingPacket>>>) {
    // Connect to the websocket
    info!("Connecting to websocket");

    let url = Url::parse("ws://echo.websocket.org/").unwrap();
    let (socket, _) = tokio_tungstenite::connect_async(url).await.unwrap();

    // Split the stream into its component sender and receiver
    let (tx, rx) = socket.split();

    // Forward the outgoing packets to the websocket
    let out_channel = outbound_stream.map(|e|
        Message::Text(
            serde_json::to_string(&OutboundPacket::from(e))
                .unwrap())
    )
        .map(Ok)
        .forward(tx);

    // Push to incoming packets onto the incoming queue
    let in_channel = rx.for_each(|f| {
        let input_queue = input_queue.clone();
        async move {
            match f.unwrap() {
                Message::Text(s) => {
                    let incoming_packet = IncomingPacket::deserialize(&s).await;
                    debug!("Received packet {:?}", incoming_packet);
                    input_queue.lock().await.push_back(incoming_packet);
                }
                Message::Close(_) => {}
                _ => {
                }
            }
        }
    });

    // Win.
    pin_mut!(out_channel, in_channel);
    future::select(out_channel, in_channel).await;
}

#[derive(Serialize)]
struct OutboundPacket {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl From<OutgoingPacket> for OutboundPacket {
    fn from(packet: OutgoingPacket) -> Self {
        match packet {
            OutgoingPacket::KeepAlivePacket {} => Self {
                id: 0,
                name: None,
                key: None,
                reason: None,
            },
            OutgoingPacket::JoinRequestPacket { name } => Self {
                id: 1,
                name: Some(name),
                key: None,
                reason: None,
            },
            OutgoingPacket::CInputPacket { key } => Self {
                id: 2,
                name: None,
                key: Some(key),
                reason: None,
            },
            OutgoingPacket::DisconnectPacket { reason } => Self {
                id: 3,
                name: None,
                key: None,
                reason: Some(reason),
            },
        }
    }
}