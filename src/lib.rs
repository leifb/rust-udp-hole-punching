use bincode::{Encode, Decode};

pub const SERVER_PORT: usize = 39738;

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

impl Packet {
    pub fn encode(&self) -> Vec<u8> {
        let config = bincode::config::standard();
        bincode::encode_to_vec(self, config).unwrap()
    }

    pub fn decode(buffer: &[u8]) -> Packet {
        let config = bincode::config::standard();
        bincode::decode_from_slice(&buffer, config).unwrap().0
    }
}
