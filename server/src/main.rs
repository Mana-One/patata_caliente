use std::net::{Shutdown, TcpListener, TcpStream};

use common::challenge::md5_hashcash::MD5HashCashInput;
use common::domain::PublicPlayer;
use common::message::{Challenge, EndOfGame, Message, PublicLeaderBoard, Welcome};
use common::utils;
use common::utils::read_message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot bind: {err}")
    };

    println!("Listening on port 7878");
    let players: Vec<PublicPlayer> = vec![];
    let mut handle_message = message_handler_builder(players);

    for message in listener.incoming() {
        match message {
            Ok(mut stream) => {
                loop {
                    if !read_message(&mut stream, &mut handle_message) {
                        break;
                    }
                }

                // PROPERLY CLOSE CONNECTION
                match stream.shutdown(Shutdown::Both) {
                    Ok(_) => println!("Server shutdown."),
                    Err(_) => {}
                };
                break;
            }
            Err(err) => panic!("Cannot connect: {err}")
        }
    }
}

fn message_handler_builder(mut players: Vec<PublicPlayer>) -> impl FnMut(&Message, &mut TcpStream) -> bool {
    move |msg, stream| {
        println!("\n{:?}", msg);
        match msg {
            Message::Hello => {
                utils::write_message(&Message::Welcome(
                    Welcome { version: 1 }
                ), stream);
                true
            }
            Message::Welcome(_) => true,
            Message::Subscribe(subscribe) => {
                players.append(
                    &mut vec![
                        PublicPlayer {
                            name: subscribe.name.clone(),
                            stream_id: stream.peer_addr().unwrap().to_string(),
                            score: 0,
                            steps: 0,
                            is_active: true,
                            total_used_time: 0.0,
                        },
                        PublicPlayer {
                            name: "Toufik".to_string(),
                            stream_id: "toufik_url".to_string(),
                            score: 0,
                            steps: 0,
                            is_active: true,
                            total_used_time: 0.0,
                        },
                    ]
                );
                utils::write_message(&Message::PublicLeaderBoard(
                    PublicLeaderBoard(players.clone())
                ), stream);

                utils::write_message(&Message::Challenge(
                    Challenge::MD5HashCash(
                        MD5HashCashInput {
                            complexity: 10,
                            message: "è_b987b-_vbè(79B".to_string(),
                        }
                    )
                ), stream);
                true
            }
            Message::SubscribeResult(_) => true,
            Message::PublicLeaderBoard(_) => true,
            Message::Challenge(_) => true,
            Message::ChallengeResult(_) => {
                utils::write_message(&Message::EndOfGame(
                    EndOfGame {
                        leader_board: PublicLeaderBoard(players.clone()),
                    }
                ), stream);
                false
            }
            Message::RoundSummary(_) => true,
            Message::EndOfGame(_) => true
        }
    }
}