use std::io::Error;
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
                    match read_message(&mut stream, &mut handle_message) {
                        Ok(true) => {},
                        Ok(false) => { break; },
                        Err(e) => {
                            println!("{e:?}");
                            break;
                        }
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

fn message_handler_builder(mut players: Vec<PublicPlayer>) -> impl FnMut(&Message, &mut TcpStream) -> Result<bool, Error>  {
    move |msg, stream| {
        match msg {
            Message::Hello => {
                utils::write_message(&Message::Welcome(
                    Welcome { version: 1 }
                ), stream)?;
                Ok(true)
            }
            Message::Welcome(_) => Ok(true),
            Message::Subscribe(subscribe) => {
                players.append(
                    &mut vec![
                        PublicPlayer {
                            name: subscribe.name.clone(),
                            stream_id: stream.peer_addr()?.to_string(),
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
                ), stream)?;

                utils::write_message(&Message::Challenge(
                    Challenge::MD5HashCash(
                        MD5HashCashInput {
                            complexity: 10,
                            message: "è_b987b-_vbè(79B".to_string(),
                        }
                    )
                ), stream)?;
                Ok(true)
            }
            Message::SubscribeResult(_) => Ok(true),
            Message::PublicLeaderBoard(_) => Ok(true),
            Message::Challenge(_) => Ok(true),
            Message::ChallengeResult(_) => {
                utils::write_message(&Message::EndOfGame(
                    EndOfGame {
                        leader_board: PublicLeaderBoard(players.clone()),
                    }
                ), stream)?;
                Ok(false)
            }
            Message::RoundSummary(_) => Ok(true),
            Message::EndOfGame(_) => Ok(true)
        }
    }
}