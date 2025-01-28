use std::{env, fmt::format, net::UdpSocket, thread::sleep, time::Duration};

use hole_punch::Packet;

const SERVER_ADDR: &'static str = "168.119.58.166:39738";
const LOCAL_ADDR: &'static str = "0.0.0.0:39738";


fn main() {
    // Get name of this client from command line arg
    let mut args = env::args();
    let client_name = args.next().expect("Client needs at least one argument");
    let other = args.next();

    let socket = UdpSocket::bind(LOCAL_ADDR).unwrap();
    let bincode_config = bincode::config::standard();
    let mut buf = [0u8; 128];

    println!("[INFO] Client name: {}", client_name);
    println!("[INFO] Sending register packet to server on {}", SERVER_ADDR);
    let message = Packet::Register(client_name);
    let message = bincode::encode_to_vec(message, bincode_config).unwrap();
    socket.send_to(message.as_slice(), SERVER_ADDR).unwrap();

    println!("[INFO] Waiting ack...");
    let (_, _) = socket.recv_from(&mut buf).unwrap();
    let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
    match message {
        Packet::RegisterAck => {
            println!("[INFO] Sucessfully register at hole-punch server");
        },
        _ => {
            panic!("Server replied with wrong package");
        }
    }

    if let Some(other) = other {
        println!("[INFO] Trying to connect to other client");
        let message = Packet::HolePunchRequest(other);
        let message = bincode::encode_to_vec(message, bincode_config).unwrap();
        let mut other: String = String::new();
        loop {
            println!("[INFO] Sending hole punch request...");
            socket.send_to(message.as_slice(), SERVER_ADDR).unwrap();

            let (_, _) = socket.recv_from(&mut buf).unwrap();
            let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
            match message {
                Packet::HolePunchResponseOk(other_client) => {
                    println!(" > Got an OK!");
                    other = other_client;
                    break;
                },
                Packet::HolePunchResponseUnknown => {
                    println!(" > Other client is not registered. Waiting 1s");
                    sleep(Duration::from_secs(1));
                }
                _ => {
                    panic!("Server replied with wrong package");
                }
            }
        }

        println!("[INFO] Sending message directly to other client");
        let message = Packet::Message("Hello from client!".to_string());
        let message = bincode::encode_to_vec(message, bincode_config).unwrap();
        socket.send_to(message.as_slice(), other).unwrap();
        
        let (_, _) = socket.recv_from(&mut buf).unwrap();
        let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
        println!(" > Recieved {:?}", message);
    } else {
        println!("[INFO] Waiting for messages...");
        loop {
            let (_, other) = socket.recv_from(&mut buf).unwrap();
            let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
            println!("[INFO] Recieved {:?}", message);
            match message {
                Packet::Message(message) => {
                    let message = Packet::Message(format!("Reply to {}", message));
                    let message = bincode::encode_to_vec(message, bincode_config).unwrap();
                    socket.send_to(message.as_slice(), other).unwrap();
                },
                _ => {}
            }
        }
    }    
}
