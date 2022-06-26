use std::{net::{TcpStream, Shutdown}, io::{Write, Read}, env};
use serde_json;
use common::domain::ChallengeAnswer;
use common::message::{
    Message, 
    Subscribe, 
    SubscribeResult, 
    Challenge, 
    ChallengeResult,
};
use common::challenge::{MD5HashCashChallenge, Challenge as ChallengTrait};

fn main() {
    println!("Connecting to server...\n");

    let args: Vec<String> = env::args().collect();
    let default = "omniscient_adjucator";
    let username = args.get(1).map_or(default, |r| r.as_str());

    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            // SEND MSG
            write_message(&Message::Hello, &mut stream);
            
            loop {
                if !read_message(&mut stream, username) {
                    break;
                }
            }

            // PROPERLY CLOSE CONNECTION
            match stream.shutdown(Shutdown::Both) {
                Ok(_) => println!("Client shutdown."),
                Err(_) => {}
            };
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

// returns boolean indicating whether the client should keep the TCP connection alive
fn read_message(stream: &mut TcpStream, username: &str) -> bool {
    // READ SIZE OF RESPONSE
    let mut size_res = [0u8; 4];
    stream.read_exact(&mut size_res).unwrap();
    let size: u32 = u32::from_be_bytes(size_res);

    // READ DATA 
    let mut data_res: Vec<u8> = vec![0u8; size.try_into().unwrap()];
    stream.read_exact(&mut data_res).unwrap();
    let msg = serde_json::from_str::<Message>(&String::from_utf8_lossy(&data_res)).unwrap();

    handle_incoming_message(&msg, stream, username)
}

fn handle_incoming_message(msg: &Message, stream: &mut TcpStream, username: &str) -> bool {
    println!("\n{:?}", msg);
    match msg {
        Message::Welcome(welcome) => {


            write_message(&Message::Subscribe(
                Subscribe::new(username) 
            ), stream);
            true
        },
        Message::SubscribeResult(subcribe_result) => {
            // println!("{:?}", subcribe_result);
            match subcribe_result {
                SubscribeResult::Ok => true,
                SubscribeResult::Err(_) => false
            }
        },
        Message::PublicLeaderBoard(public_leader_board) => { 
            // println!("{:?}", public_leader_board);
            true
        },
        Message::Challenge(challenge) => { 
            // println!("Received {:?}", challenge);
            match challenge {
                Challenge::MD5HashCash(hash_cash) => {
                    let data = MD5HashCashChallenge::new(hash_cash.clone());
                    let answer = data.solve();
                    let target = if username == "omniscient_adjucator" 
                        { "abject_testament" } 
                        else { "omniscient_adjucator" };
                    let message = ChallengeResult::new(
                        ChallengeAnswer::MD5HashCash(answer), 
                        target
                    );
                    write_message(&Message::ChallengeResult(message), stream);
                    true
                }
            }
        },
        Message::RoundSummary(round_summary) => { 
            // println!("{:?}", round_summary);
            true 
        },
        Message::EndOfGame(end_of_game) => { 
            // println!("{:?}", end_of_game);
            false 
        },
        _ => {
            println!("Invalid message {:?}", msg);
            false
        }
    }
}