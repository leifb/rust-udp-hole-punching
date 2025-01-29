use std::{env, net::UdpSocket, thread::sleep, time::Duration};

use hole_punch::{Packet, SERVER_PORT};

const LOCAL_ADDR: &'static str = "0.0.0.0:0";

fn main() {
    // Get command line args
    let mut args = env::args().skip(1);
    let server_ip = args.next().expect("Missing argument for server address");
    let local_client_name = args.next().expect("Missing argument for client name");
    let remote_client_name = args.next(); // Second client name is optional
    
    let server = format!("{}:{}", server_ip, SERVER_PORT);
    let socket = UdpSocket::bind(LOCAL_ADDR).unwrap();
    let mut buf = [0u8; 128];

    println!("[INFO] Server: {}", server_ip);
    println!("[INFO] This client: {}", local_client_name);
    println!("[INFO] Other client: {:?}", remote_client_name);
    println!("[INFO] Sending register packet to server on {}", server);
    let message = Packet::Register(local_client_name.clone()).encode();
    socket.send_to(message.as_slice(), &server).unwrap();

    println!(" > Waiting ack...");
    let (_, _) = socket.recv_from(&mut buf).unwrap();
    let message = Packet::decode(&buf);
    match message {
        Packet::RegisterAck => {
            println!(" > Successfully registered at hole-punch server");
        },
        _ => {
            panic!("Server replied with wrong package");
        }
    }

    if let Some(remote_client_name) = remote_client_name {
        // If this is the client requesting the hole punch
        println!("[INFO] Requesting hole punch to other client");
        let message = Packet::HolePunchRequest(remote_client_name).encode();
        let remote_client_address: String;
        loop {
            println!(" > Sending hole punch request...");
            socket.send_to(message.as_slice(), &server).unwrap();

            let (_, _) = socket.recv_from(&mut buf).unwrap();
            let message = Packet::decode(&buf);
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
        // If this is the client waiting for the hole punch
        println!("[INFO] Waiting for hole punch to start...");
        let remote_client_address: String;
        loop {
            let (_, _) = socket.recv_from(&mut buf).unwrap();
            let message = Packet::decode(&buf);
            println!(" > Received {:?}", message);
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
    let mut buf = [0u8; 128];
    socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

    loop {
        let message = Packet::Message(format!("Hello from {local_client_name}")).encode();
        socket.send_to(message.as_slice(), &remote_client_address).unwrap();
        
        match socket.recv_from(&mut buf) {
            Ok(_) => {
                let message = Packet::decode(&buf);
                println!(" > Received {:?}", message);
            },
            Err(_) => {
                println!(" > no response yet");
            }
        }

        sleep(Duration::from_secs(1));
    }
}