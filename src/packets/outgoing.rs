#[derive(Clone, Debug)]
pub enum OutgoingPacket {
    KeepAlivePacket {},
    JoinRequestPacket { name: String },
    CInputPacket { key: String },
    DisconnectPacket { reason: String },
}

impl OutgoingPacket {
    pub fn id(&self) -> i32 {
        match self {
            OutgoingPacket::KeepAlivePacket {} => 0,
            OutgoingPacket::JoinRequestPacket { name: _ } => 1,
            OutgoingPacket::CInputPacket { key: _ } => 2,
            OutgoingPacket::DisconnectPacket { reason: _ } => 3,
        }
    }
}
