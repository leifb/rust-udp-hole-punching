use std::net::UdpSocket;

use hole_punch::Message;

const SERVER_ADDR: &'static str = "168.119.58.166:39738";
const LOCAL_ADDR: &'static str = "0.0.0.0:39738";


fn main() {
    let socket = UdpSocket::bind(LOCAL_ADDR).unwrap();
    let bincode_config = bincode::config::standard();

    println!("[INFO] Sending package to server on {}", SERVER_ADDR);
    let message = Message::String(("Response!").to_string());
    let message = bincode::encode_to_vec(message, bincode_config).unwrap();
    socket.send_to(message.as_slice(), SERVER_ADDR).unwrap();

    println!("[INFO] Waiting for response...");
    let mut buf = [0u8; 128];
    
    let (_, _) = socket.recv_from(&mut buf).unwrap();
    let (message, _): (Message, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
    println!("[RECIEVED] {:?}", message);
}