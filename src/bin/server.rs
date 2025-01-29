use std::{collections::HashMap, net::UdpSocket};

use hole_punch::{Packet, SERVER_PORT};

fn main() {
    let address = format!("0.0.0.0:{}", SERVER_PORT);
    println!("[INFO] Starting server on {}", address);
    let socket = UdpSocket::bind(address).unwrap();

    let mut registered_clients: HashMap<String, String> = HashMap::new();
    let mut buffer = [0u8; 128];
    loop {
        let (_, client_address) = socket.recv_from(&mut buffer).unwrap();
        let message = Packet::decode(&buffer);
        println!("[RECIEVED] {:?}", message);

        match message {
            Packet::Register(name) => {
                let address = client_address.to_string();
                println!(" > Storing client {} as {}", name, address);
                registered_clients.insert(name, address);

                let response = Packet::RegisterAck.encode();
                socket.send_to(response.as_slice(), client_address).unwrap();        
            },
            Packet::HolePunchRequest(client_b_name) => {
                // Check if the requesting client is registered
                let address = client_address.to_string();
                let Some(client_a) = registered_clients.iter().find(|(_, a)| a == &&address) else {
                    println!(" > Client that is not registered tried to hole punch.");
                    let response = Packet::HolePunchResponseUnknown.encode();
                    socket.send_to(response.as_slice(), client_address).unwrap();  
                    continue;
                };

                // Check if the other client is registered
                let Some(client_b_address) = registered_clients.get(&client_b_name) else {
                    println!(" > Other client is not registered.");
                    let response = Packet::HolePunchResponseUnknown.encode();
                    socket.send_to(response.as_slice(), client_address).unwrap();  
                    continue;
                };

                // Initiate the hole punch by giving both clients each others address
                let request = Packet::HolePunchInitiate {
                    client_name: client_a.0.clone(),
                    client_address: client_a.1.clone(),
                }.encode();
                socket.send_to(request.as_slice(), client_b_address).unwrap();

                let request = Packet::HolePunchInitiate {
                    client_name: client_b_name,
                    client_address: client_b_address.clone(),
                }.encode();
                socket.send_to(request.as_slice(), client_address).unwrap();
            },
            _ => {},
        }
    }
}
