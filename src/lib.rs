use bincode::{Encode, Decode};

#[derive(Encode, Decode, Debug)]
pub enum Packet {
    Message(String),
    Register(String),
    RegisterAck,
    /// Requests starting a hole punch.
    HolePunchRequest(String),
    /// The cannot initiate a hole punch because the client
    /// it not known.
    HolePunchResponseUnknown,
    HolePunchInitiate{
        client_name: String,
        client_address: String,
    },
}