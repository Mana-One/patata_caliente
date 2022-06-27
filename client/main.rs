use clap::{Arg, App, SubCommand};
use std::net::{TcpStream, Shutdown};
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

fn main() {
    let app = App::new("La patata caliente")
        .about("A client app for 'La patate chaude'... but it's in a Spanish name.")
        .author("Paul Barrié | Paolo Manaois | Adem Mrizak")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(SubCommand::with_name("run")
            .about("Runs the client")
            .arg_required_else_help(true)
            .arg(Arg::new("username")
                .short('u')
                .takes_value(true)
                .value_name("USERNAME")));

    let matches = app.get_matches();
    match matches.subcommand() {
        Some(("run", run_args)) => {
            if let Some(username) = run_args.get_one::<String>("username") {
                launch_client(username.to_string());
            }
        },
        _ => unreachable!("Exhausted list of subcommands")
    }
}

fn launch_client(username: String) {
    println!("Connecting to server...\n");

    let stream = TcpStream::connect("localhost:7878");
    let players: Vec<PublicPlayer> = vec![];
    let mut handle_message = message_handler_builder(username.to_string(), players);

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

fn message_handler_builder(username: String, mut players: Vec<PublicPlayer>) -> utils::MessageHandler {
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
                // println!("Received {:?}", challenge);
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