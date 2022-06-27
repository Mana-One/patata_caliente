use std::net::{TcpStream, Shutdown};
use std::env;
use common::domain::ChallengeAnswer;
use common::message::{
    Message, 
    Subscribe, 
    SubscribeResult, 
    Challenge, 
    ChallengeResult,
};
use common::challenge::{md5_hashcash::MD5HashCashChallenge, Challenge as ChallengTrait};
use common::utils;

fn main() {
    println!("Connecting to server...\n");

    let args: Vec<String> = env::args().collect();
    let username = args.get(1).expect("Missing username !");

    let stream = TcpStream::connect("localhost:7878");
    let mut handle_message = message_handler_builder(username.to_string());

    match stream {
        Ok(mut stream) => {
            // SEND MSG
            utils::write_message(&Message::Hello, &mut stream);
            
            loop {
                if !utils::read_message(&mut stream, &mut handle_message) {
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

fn message_handler_builder(username: String) -> utils::MessageHandler {
    Box::new(move |msg, stream| {
        println!("\n{:?}", msg);
        match msg {
            Message::Welcome(_welcome) => {
                utils::write_message(&Message::Subscribe(
                    Subscribe::new(username.as_str()) 
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
            Message::PublicLeaderBoard(_public_leader_board) => { 
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
                        utils::write_message(&Message::ChallengeResult(message), stream);
                        true
                    }
                }
            },
            Message::RoundSummary(_round_summary) => { 
                // println!("{:?}", round_summary);
                true 
            },
            Message::EndOfGame(_end_of_game) => { 
                // println!("{:?}", end_of_game);
                false 
            },
            _ => {
                println!("Invalid message {:?}", msg);
                false
            }
        }
    })
}