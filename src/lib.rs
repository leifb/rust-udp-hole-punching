use bincode::{Encode, Decode};

#[derive(Encode, Decode, Debug)]
pub enum Packet {
    Message(String),
    Register(String),
    RegisterAck,
    HolePunchRequest(String),
    HolePunchResponseUnknown,
    HolePunchResponseOk(String),
}