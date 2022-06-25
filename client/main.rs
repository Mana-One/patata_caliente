use std::{net::{TcpStream, Shutdown}, io::{Write, Read}};
use serde_json;
use common::{Message};

fn main() {
    println!("Connecting to server...\n");
    let message = serde_json::to_string(&Message::Hello).unwrap();
    let message = message.as_bytes();

    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            // SEND MSG
            let message_size = message.len() as u32;
            stream.write_all(&message_size.to_be_bytes()).unwrap();
            stream.write_all(message).unwrap();
            
            // READ SIZE OF RESPONSE
            let mut size_res = [0u8; 4];
            stream.read_exact(&mut size_res).unwrap();
            let size: u32 = u32::from_be_bytes(size_res);

            // READ DATA 
            let mut data_res: Vec<u8> = vec![0u8; size.try_into().unwrap()];
            stream.read_exact(&mut data_res).unwrap();
            let msg = serde_json::from_str::<Message>(&String::from_utf8_lossy(&data_res));
            println!("Received {:?}\n", msg);

            // PROPERLY CLOSE CONNECTION
            stream.shutdown(Shutdown::Both).expect("Error while shutting down.");
        },
        Err(e) => {
            println!("Cannot connect to server: {}", e);
        }
    }
}