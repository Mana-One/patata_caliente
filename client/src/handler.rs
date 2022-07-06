use std::io::Error;
use std::net::{Shutdown, TcpStream};
use common::challenge::monstrous_maze::MonstrousMazeChallenge;
use common::domain::{ChallengeAnswer, PublicPlayer};
use common::message::{
    Message, 
    Subscribe, 
    SubscribeResult, 
    Challenge, 
    ChallengeResult,
};
use common::challenge::{md5_hashcash::MD5HashCashChallenge, Challenge as ChallengTrait};
use common::utils;
use crate::target_strategy::select_target;

pub fn message_handler_builder(username: String, mut players: Vec<PublicPlayer>) -> 
    impl FnMut(&Message, &mut TcpStream) -> Result<bool, Error> {

    move |msg, stream| {
        println!("{msg:?}");
        match msg {
            Message::Welcome(_welcome) => {
                utils::write_message(&Message::Subscribe(
                    Subscribe::new(username.as_str()) 
                ), stream)?;
                Ok(true)
            },
            Message::SubscribeResult(subcribe_result) => {
                match subcribe_result {
                    SubscribeResult::Ok => Ok(true),
                    SubscribeResult::Err(_) => Ok(false)
                }
            },
            Message::PublicLeaderBoard(public_leader_board) => { 
                players.append(&mut public_leader_board.0
                    .clone()
                    .into_iter()
                    .filter(|p| p.name != username)
                    .collect::<Vec<PublicPlayer>>()
                );
                
                Ok(!players.is_empty())
            },
            Message::Challenge(challenge) => {
                match challenge {
                    Challenge::MD5HashCash(hash_cash) => {
                        let data = MD5HashCashChallenge::new(hash_cash.clone());
                        let answer = data.solve();

                        let target = select_target(&players);

                        if let Some(target) = target {
                            let message = ChallengeResult::new(
                                ChallengeAnswer::MD5HashCash(answer), 
                                target.as_str()
                            );
                            utils::write_message(&Message::ChallengeResult(message), stream)?;
                            return Ok(true);
                        }

                        Ok(false)
                    },

                    Challenge::MonstrousMaze(maze) => {
                        let data = MonstrousMazeChallenge::new(maze.clone());
                        let answer = data.solve();

                        let target = select_target(&players);

                        if let None = target {
                            return Ok(false);
                        }

                        let message = ChallengeResult::new(
                            ChallengeAnswer::MonstrousMaze(answer), 
                            target.unwrap().as_str()
                        );
                        utils::write_message(&Message::ChallengeResult(message), stream)?;
                        Ok(true)
                    }
                }
            },
            Message::RoundSummary(_round_summary) => Ok(true),
            Message::EndOfGame(_end_of_game) => { 
                // PROPERLY CLOSE CONNECTION
                match stream.shutdown(Shutdown::Both) {
                    Ok(_) => println!("Client shutdown."),
                    Err(_) => {}
                };
                Ok(false)
            },
            _ => {
                println!("Invalid message {:?}", msg);
                Ok(false)
            }
        }
    }
}