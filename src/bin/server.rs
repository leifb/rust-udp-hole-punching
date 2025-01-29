use std::{collections::HashMap, net::UdpSocket};

use hole_punch::Packet;

const SERVER_ADDR: &'static str = "0.0.0.0:39738";

fn main() {
    println!("[INFO] Starting server on {}", SERVER_ADDR);
    let socket = UdpSocket::bind(SERVER_ADDR).unwrap();

    let mut registered_clients: HashMap<String, String> = HashMap::new();

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
                registered_clients.insert(name, address);

                let response = Packet::RegisterAck;
                let response = bincode::encode_to_vec(response, bincode_config).unwrap();
                socket.send_to(response.as_slice(), client_address).unwrap();        
            },
            Packet::HolePunchRequest(client_b_name) => {
                // Check if the requesting client is registered
                let address = client_address.to_string();
                let Some(client_a) = registered_clients.iter().find(|(_, a)| a == &&address) else {
                    println!(" > Client that is not registered tried to hole punch.");
                    continue;
                };

                // Check if the other client is registered
                let Some(client_b_address) = registered_clients.get(&client_b_name) else {
                    println!(" > Other client is not registered.");
                    let response = Packet::HolePunchResponseUnknown;
                    let response = bincode::encode_to_vec(response, bincode_config).unwrap();
                    socket.send_to(response.as_slice(), client_address).unwrap();  
                    continue;
                };

                // Initiate the hole punch by giving both clients each others address
                let request = Packet::HolePunchInitiate {
                    client_name: client_a.0.clone(),
                    client_address: client_a.1.clone(),
                };
                let request = bincode::encode_to_vec(request, bincode_config).unwrap();
                socket.send_to(request.as_slice(), client_b_address).unwrap();

                let request = Packet::HolePunchInitiate {
                    client_name: client_b_name,
                    client_address: client_b_address.clone(),
                };
                let request = bincode::encode_to_vec(request, bincode_config).unwrap();
                socket.send_to(request.as_slice(), client_address).unwrap();
            },
            _ => {},
        }
    }
}
