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

pub fn message_handler_builder(username: String, mut players: Vec<PublicPlayer>) -> impl FnMut(&Message, &mut TcpStream) -> bool {
    move |msg, stream| {
        println!("{msg:?}");
        match msg {
            Message::Welcome(_welcome) => {
                utils::write_message(&Message::Subscribe(
                    Subscribe::new(username.as_str()) 
                ), stream);
                true
            },
            Message::SubscribeResult(subcribe_result) => {
                match subcribe_result {
                    SubscribeResult::Ok => true,
                    SubscribeResult::Err(_) => false
                }
            },
            Message::PublicLeaderBoard(public_leader_board) => { 
                players.append(&mut public_leader_board.0
                    .clone()
                    .into_iter()
                    .filter(|p| p.name != username)
                    .collect::<Vec<PublicPlayer>>()
                );
                
                !players.is_empty()
            },
            Message::Challenge(challenge) => {
                match challenge {
                    Challenge::MD5HashCash(hash_cash) => {
                        let data = MD5HashCashChallenge::new(hash_cash.clone());
                        let answer = data.solve();

                        let mut sorted_players = players.clone();
                        sorted_players.sort_by(|a, b| a.score.cmp(&b.score));
                        let target = sorted_players.get(0).map(|p| p.name.as_str());
                        if let None = target {
                            return false;
                        }

                        let message = ChallengeResult::new(
                            ChallengeAnswer::MD5HashCash(answer), 
                            target.unwrap()
                        );
                        utils::write_message(&Message::ChallengeResult(message), stream);
                        true
                    },

                    Challenge::MonstrousMaze(maze) => {
                        let data = MonstrousMazeChallenge::new(maze.clone());
                        let answer = data.solve();

                        let mut sorted_players = players.clone();
                        sorted_players.sort_by(|a, b| a.score.cmp(&b.score));
                        let target = sorted_players.get(0).map(|p| p.name.as_str());
                        if let None = target {
                            return false;
                        }

                        let message = ChallengeResult::new(
                            ChallengeAnswer::MonstrousMaze(answer), 
                            target.unwrap()
                        );
                        utils::write_message(&Message::ChallengeResult(message), stream);
                        true
                    }
                }
            },
            Message::RoundSummary(_round_summary) => {
                true 
            },
            Message::EndOfGame(_end_of_game) => { 
                // PROPERLY CLOSE CONNECTION
                match stream.shutdown(Shutdown::Both) {
                    Ok(_) => println!("Client shutdown."),
                    Err(_) => {}
                };
                false 
            },
            _ => {
                println!("Invalid message {:?}", msg);
                false
            }
        }
    }
}