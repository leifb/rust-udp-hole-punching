use std::net::UdpSocket;

use hole_punch::Message;

const SERVER_ADDR: &'static str = "0.0.0.0:39738";

fn main() {
    println!("[INFO] Starting server on {}", SERVER_ADDR);
    let socket = UdpSocket::bind(SERVER_ADDR).unwrap();

    let bincode_config = bincode::config::standard();
    let mut buf = [0u8; 128];
    loop {
        let (_, src_addr) = socket.recv_from(&mut buf).unwrap();
        let (message, _): (Message, usize) = bincode::decode_from_slice(&buf, bincode_config).unwrap();
        println!("[RECIEVED] {:?}", message);

        let response = Message::String(("Response!").to_string());
        let response = bincode::encode_to_vec(response, bincode_config).unwrap();
        socket.send_to(response.as_slice(), src_addr).unwrap();
    }
}
