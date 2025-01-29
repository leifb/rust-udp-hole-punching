use std::{env, net::UdpSocket, thread::sleep, time::Duration};

use hole_punch::Packet;

const SERVER_ADDR: &'static str = "168.119.58.166:39738";
const LOCAL_ADDR: &'static str = "0.0.0.0:0";


fn main() {
    // Get name of this client from command line arg
    let mut args = env::args().skip(1);
    let local_client_name = args.next().expect("Client needs at least one argument");
    let remote_client_name = args.next();

    let socket = UdpSocket::bind(LOCAL_ADDR).unwrap();
    println!("[INFO] Using loclat port: {}", socket.local_addr().unwrap().port());
    let bincode_config = bincode::config::standard();
    let mut buf = [0u8; 128];

    println!("[INFO] This client: {}", local_client_name);
    println!("[INFO] Other client: {:?}", remote_client_name);
    println!("[INFO] Sending register packet to server on {}", SERVER_ADDR);
    let message = Packet::Register(local_client_name.clone());
    let message = bincode::encode_to_vec(message, bincode_config).unwrap();
    socket.send_to(message.as_slice(), SERVER_ADDR).unwrap();

    println!(" > Waiting ack...");
    let (_, _) = socket.recv_from(&mut buf).unwrap();
    let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
    match message {
        Packet::RegisterAck => {
            println!(" > Sucessfully register at hole-punch server");
        },
        _ => {
            panic!("Server replied with wrong package");
        }
    }

    if let Some(remote_client_name) = remote_client_name {
        println!("[INFO] Requesting hole punch to other client");
        let message = Packet::HolePunchRequest(remote_client_name);
        let message = bincode::encode_to_vec(message, bincode_config).unwrap();
        let remote_client_address: String;
        loop {
            println!(" > Sending hole punch request...");
            socket.send_to(message.as_slice(), SERVER_ADDR).unwrap();

            let (_, _) = socket.recv_from(&mut buf).unwrap();
            let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
            match message {
                Packet::HolePunchInitiate { client_name, client_address } => {
                    println!(" > OK! Initiating hole punch to {}!", client_name);
                    remote_client_address = client_address;
                    break;
                },
                Packet::HolePunchResponseUnknown => {
                    println!(" > Other client is not registered. Waiting 2s");
                    sleep(Duration::from_secs(2));
                }
                _ => {
                    panic!("Server replied with wrong package");
                }
            }
        }

        send_messages_to_other_client(
            socket,
            remote_client_address,
            local_client_name,
        );
    } else {
        println!("[INFO] Waiting for hole punch to start...");
        let remote_client_address: String;
        loop {
            let (_, _) = socket.recv_from(&mut buf).unwrap();
            let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
            println!(" > Recieved {:?}", message);
            match message {
                Packet::HolePunchInitiate { client_name, client_address } => {
                    println!(" > OK! Initiating hole punch to {}!", client_name);
                    remote_client_address = client_address;
                    break;
                },
                _ => {}
            }
        }

        send_messages_to_other_client(
            socket,
            remote_client_address,
            local_client_name,
        );
    }    
}

fn send_messages_to_other_client(
    socket: UdpSocket,
    remote_client_address: String,
    local_client_name: String,
) {
    println!(" > Sending message to other client at {}", remote_client_address);
    let bincode_config = bincode::config::standard();
    let mut buf = [0u8; 128];
    socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

    loop {
        let message = Packet::Message(format!("Hello from {local_client_name}"));
        let message = bincode::encode_to_vec(message, bincode_config).unwrap();
        socket.send_to(message.as_slice(), &remote_client_address).unwrap();
        
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                let (message, _): (Packet, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
                println!(" > Recieved {:?}", message);
            },
            Err(_) => {
                println!(" > no response yet");
            }
        }
    }
}