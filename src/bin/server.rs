use std::{collections::HashMap, net::UdpSocket};

use hole_punch::Packet;

const SERVER_ADDR: &'static str = "0.0.0.0:39738";

fn main() {
    println!("[INFO] Starting server on {}", SERVER_ADDR);
    let socket = UdpSocket::bind(SERVER_ADDR).unwrap();

    let mut clients: HashMap<String, String> = HashMap::new();
    let bincode_config = bincode::config::standard();
    let mut buf = [0u8; 128];
    loop {
        let (_, client_address) = socket.recv_from(&mut buf).unwrap();
        let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
        println!("[RECIEVED] {:?}", message);

        match message {
            Packet::Register(name) => {
                let address = client_address.to_string();
                println!(" > Storing client {} as {}", name, address);
                clients.insert(name, address);

                let response = Packet::RegisterAck;
                let response = bincode::encode_to_vec(response, bincode_config).unwrap();
                socket.send_to(response.as_slice(), client_address).unwrap();        
            },
            Packet::HolePunchRequest(other) => {
                // Check if the requesting client is registered
                let address = client_address.to_string();
                let Some(client_a) = clients.iter().find(|(_, a)| a == &&address) else {
                    println!(" > Client that is not registered tried to hole punch.");
                    continue;
                };

                // Check if the other client is registered
                let Some(client_b) = clients.get(&other) else {
                    println!(" > Other client is not registered.");
                    let response = Packet::HolePunchResponseUnknown;
                    let response = bincode::encode_to_vec(response, bincode_config).unwrap();
                    socket.send_to(response.as_slice(), client_address).unwrap();  
                    continue;
                };

                // Punch a hole!
                println!(" > Punching a hole between {} and {}.", client_a.0, other);
                let response = Packet::HolePunchResponseOk(client_b.clone());
                let response = bincode::encode_to_vec(response, bincode_config).unwrap();
                socket.send_to(response.as_slice(), client_address).unwrap();
            }
            _ => {},
        }
    }
}
