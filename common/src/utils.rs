use std::net::TcpStream;
use std::io::{Read, Write};
use serde_json;
use crate::message::Message;

pub fn write_message(message: &Message, stream: &mut TcpStream) {
    let json = serde_json::to_string(message).unwrap();
    let json = json.as_bytes();

    let message_size = json.len() as u32;
    stream.write_all(&message_size.to_be_bytes()).unwrap();
    stream.write_all(json).unwrap();
}

pub fn read_message<F>(stream: &mut TcpStream, handle_message: &mut F) -> bool 
    where F : FnMut(&Message, &mut TcpStream) -> bool 
{
    // READ SIZE OF RESPONSE
    let mut size_res = [0u8; 4];
    stream.read_exact(&mut size_res).unwrap();
    let size: u32 = u32::from_be_bytes(size_res);

    // READ DATA 
    let mut data_res: Vec<u8> = vec![0u8; size.try_into().unwrap()];
    stream.read_exact(&mut data_res).unwrap();
    let msg = serde_json::from_str::<Message>(&String::from_utf8_lossy(&data_res)).unwrap();

    handle_message(&msg, stream)
}