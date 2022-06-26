use std::{net::{TcpStream, Shutdown}, io::{Write, Read}};
use serde_json;
use common::{Message, Subscribe};

fn main() {
    println!("Connecting to server...\n");

    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            // SEND MSG
            write_message(&Message::Hello, &mut stream);
            
            loop {
                if !read_message(&mut stream) {
                    break;
                }
            }

            // PROPERLY CLOSE CONNECTION
            stream.shutdown(Shutdown::Both).expect("Error while shutting down.");
        },
        Err(e) => {
            println!("Cannot connect to server: {}", e);
        }
    }
}

fn write_message(message: &Message, stream: &mut TcpStream) {
    let json = serde_json::to_string(message).unwrap();
    let json = json.as_bytes();

    let message_size = json.len() as u32;
    stream.write_all(&message_size.to_be_bytes()).unwrap();
    stream.write_all(json).unwrap();
}

// returns boolean indicating wether the client should keep the TCP connection alive
fn read_message(stream: &mut TcpStream) -> bool {
    // READ SIZE OF RESPONSE
    let mut size_res = [0u8; 4];
    stream.read_exact(&mut size_res).unwrap();
    let size: u32 = u32::from_be_bytes(size_res);

    // READ DATA 
    let mut data_res: Vec<u8> = vec![0u8; size.try_into().unwrap()];
    stream.read_exact(&mut data_res).unwrap();
    let msg = serde_json::from_str::<Message>(&String::from_utf8_lossy(&data_res)).unwrap();
    println!("Received {:?}\n", msg);

    match msg {
        Message::Welcome(welcome) => {
            println!("{:?}", welcome);
            write_message(&Message::Subscribe(
                Subscribe::new("omniscient_adjucator") 
            ), stream);
            true
        },
        Message::SubscribeResult(subcribe_result) => {
            println!("{:?}", subcribe_result);
            false
        },
        Message::PublicLeaderBoard(public_leader_board) => { 
            println!("{:?}", public_leader_board);
            false 
        },
        Message::Challenge(challenge) => { 
            println!("{:?}", challenge);
            false 
        },
        Message::RoundSummary(round_summary) => { 
            println!("{:?}", round_summary);
            false 
        },
        Message::EndOfGame(end_of_game) => { 
            println!("{:?}", end_of_game);
            false 
        },
        _ => {
            println!("Invalid message {:?}", msg);
            false
        }
    }
}