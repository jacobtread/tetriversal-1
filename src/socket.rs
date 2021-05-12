use futures::prelude::*;
use log::{info, debug, error};
use crate::util::logging::init_log;
use url::Url;
use tokio_tungstenite::tungstenite::Message;
use futures::channel::oneshot::channel;
use futures::channel::mpsc::{Sender, Receiver};
use crate::packets::outgoing::OutgoingPackets;
use crate::packets::incoming::IncomingPacket;
use serde::Serialize;
use futures_util::{future, pin_mut, StreamExt};

pub async fn create_socket(outbound_stream: Receiver<OutgoingPackets>, mut inbound_stream: Sender<String>) {
    tokio::spawn(async {
        info!("Connecting to websocket");

        let url = Url::parse("ws://echo.websocket.org/").unwrap();
        let (socket, _) = tokio_tungstenite::connect_async(url).await.unwrap();
        let (mut tx, rx) = socket.split();

        let out_channel = outbound_stream.map(|e| Message::Text(serde_json::to_string(&OutboundPacket::from(e)).unwrap()))
            .map(Ok)
            .forward(tx);

        let in_channel = rx.for_each(|f| async move {
            info!("{:?}", f.unwrap());
        });

        pin_mut!(out_channel, in_channel);
        future::select(out_channel, in_channel).await;
    });
}

#[derive(Serialize)]
struct OutboundPacket {
    id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>
}

impl From<OutgoingPackets> for OutboundPacket {
    fn from(packet: OutgoingPackets) -> Self {
        match packet  {
            OutgoingPackets::KeepAlivePacket {} => Self {
                id: 0,
                name: None,
                key: None,
                reason: None
            },
            OutgoingPackets::JoinRequestPacket { name } => Self {
                id: 1,
                name: Some(name),
                key: None,
                reason: None
            },
            OutgoingPackets::CInputPacket { key } => Self {
                id: 2,
                name: None,
                key: Some(key),
                reason: None
            },
            OutgoingPackets::DisconnectPacket { reason} => Self {
                id: 3,
                name: None,
                key: None,
                reason: Some(reason)
            },
        }
    }
}