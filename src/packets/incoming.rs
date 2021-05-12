use serde::{Deserialize, Serialize};

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
    score: Option<i32> 
}

pub enum IncommingPacket {
    KeepAlivePacket {},
    JoinResponsePacket {uuid: String},
    JoinFailurePacket {reason: String},
    DisconnectPacket {reason: String},
    PlayerJoinPacket {name: String, uuid: String},
    PlayerLeavePacket {reason: String, uuid: String},
    PlayPacket {},
    TimeTillStart {time: String},
    StopPacket  {},
    ControlPacket {},
    ControlsPacket {name: String},
    BulkMapPacket {lines: Vec<String>},
    ActivePiecePacket {tile: Vec<Vec<i32>>},
    NextPiecePacket {tile: Vec<Vec<i32>>},
    MoveActivePacket {x: i32, y: i32},
    RotateActivePacket {},
    ScoreUpdatePacket {score: i32},
    InvaldPacket {}
}

impl From<&String> for IncommingPacket {
    fn from(packet_src: &String) -> Self {
        let packet: BasePacket = serde_json::from_str(packet_src.as_str()).unwrap();
        match packet.id {
            0 => Self::KeepAlivePacket {},
            1 => Self::JoinResponsePacket {uuid: packet.uuid.unwrap() },
            2 => Self::JoinFailurePacket {reason: packet.reason.unwrap() },
            3 => Self::DisconnectPacket {reason: packet.reason.unwrap() },
            4 => Self::PlayerJoinPacket {uuid: packet.uuid.unwrap(), name: packet.name.unwrap()},
            5 => Self::PlayerLeavePacket {uuid: packet.uuid.unwrap(), reason: packet.reason.unwrap()},
            6 => Self::PlayPacket {},
            7 => Self::TimeTillStart {time: packet.time.unwrap()},
            8 => Self::StopPacket {},
            9 => Self::ControlPacket {},
            10 => Self::ControlsPacket {name: packet.name.unwrap()},
            11 => Self::BulkMapPacket {lines: packet.lines.unwrap()},
            12 => Self::ActivePiecePacket {tile: packet.tile.unwrap()},
            13 => Self::NextPiecePacket {tile: packet.tile.unwrap()},
            14 => Self::MoveActivePacket {x: packet.x.unwrap(), y: packet.y.unwrap()},
            15 => Self::RotateActivePacket {},
            16 => Self::ScoreUpdatePacket {score: packet.score.unwrap()},
            _ => Self::InvaldPacket {}, 
        }
    }
}
