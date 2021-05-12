use serde::{Deserialize, Serialize};
use futures::FutureExt;
use crate::packets::incoming::IncomingPacket::InvalidPacket;
use std::panic::{set_hook, take_hook};

#[derive(Serialize, Deserialize)]
struct BasePacket {
    id: i32,
    uuid: Option<String>,
    reason: Option<String>,
    name: Option<String>,
    time: Option<String>,
    lines: Option<Vec<String>>,
    tile: Option<Vec<Vec<i32>>>,
    x: Option<i32>,
    y: Option<i32>,
    score: Option<i32>,
}

#[derive(Clone, Debug)]
pub enum IncomingPacket {
    KeepAlivePacket {},
    JoinResponsePacket { uuid: String },
    JoinFailurePacket { reason: String },
    DisconnectPacket { reason: String },
    PlayerJoinPacket { name: String, uuid: String },
    PlayerLeavePacket { reason: String, uuid: String },
    PlayPacket {},
    TimeTillStart { time: String },
    StopPacket {},
    ControlPacket {},
    ControlsPacket { name: String },
    BulkMapPacket { lines: Vec<String> },
    ActivePiecePacket { tile: Vec<Vec<i32>> },
    NextPiecePacket { tile: Vec<Vec<i32>> },
    MoveActivePacket { x: i32, y: i32 },
    RotateActivePacket {},
    ScoreUpdatePacket { score: i32 },
    InvalidPacket {source: String},
}

impl IncomingPacket {
    pub async fn deserialize(packet_src: &String) -> Self {
        let og = take_hook();

        let result = async {
            set_hook(Box::new(|_| {}));
            let packet: BasePacket = serde_json::from_str(packet_src.as_str()).unwrap();

            match packet.id {
                0 => Self::KeepAlivePacket {},
                1 => Self::JoinResponsePacket {
                    uuid: packet.uuid.unwrap(),
                },
                2 => Self::JoinFailurePacket {
                    reason: packet.reason.unwrap(),
                },
                3 => Self::DisconnectPacket {
                    reason: packet.reason.unwrap(),
                },
                4 => Self::PlayerJoinPacket {
                    uuid: packet.uuid.unwrap(),
                    name: packet.name.unwrap(),
                },
                5 => Self::PlayerLeavePacket {
                    uuid: packet.uuid.unwrap(),
                    reason: packet.reason.unwrap(),
                },
                6 => Self::PlayPacket {},
                7 => Self::TimeTillStart {
                    time: packet.time.unwrap(),
                },
                8 => Self::StopPacket {},
                9 => Self::ControlPacket {},
                10 => Self::ControlsPacket {
                    name: packet.name.unwrap(),
                },
                11 => Self::BulkMapPacket {
                    lines: packet.lines.unwrap(),
                },
                12 => Self::ActivePiecePacket {
                    tile: packet.tile.unwrap(),
                },
                13 => Self::NextPiecePacket {
                    tile: packet.tile.unwrap(),
                },
                14 => Self::MoveActivePacket {
                    x: packet.x.unwrap(),
                    y: packet.y.unwrap(),
                },
                15 => Self::RotateActivePacket {},
                16 => Self::ScoreUpdatePacket {
                    score: packet.score.unwrap(),
                },
                _ => Self::InvalidPacket {
                    source: packet_src.to_string()
                },
            }
        }.catch_unwind().await;

        set_hook(og);

        match result {
            Ok(e) => e,
            _ => { Self::InvalidPacket {
                source: packet_src.to_string()
            }}
        }
    }
}
