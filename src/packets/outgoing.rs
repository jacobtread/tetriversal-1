#[derive(Clone, Debug)]
pub enum OutgoingPackets {
    KeepAlivePacket {},
    JoinRequestPacket { name: String },
    CInputPacket { key: String },
    DisconnectPacket { reason: String },
}

impl OutgoingPackets {
    pub fn id(&self) -> i32 {
        match self {
            OutgoingPackets::KeepAlivePacket {} => 0,
            OutgoingPackets::JoinRequestPacket { name: _ } => 1,
            OutgoingPackets::CInputPacket { key: _ } => 2,
            OutgoingPackets::DisconnectPacket { reason: _ } => 3,
        }
    }
}
