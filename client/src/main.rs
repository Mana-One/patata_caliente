use clap::{Arg, App, SubCommand};
use std::net::TcpStream;
use common::domain::PublicPlayer;
use common::message::Message;
use common::utils;

mod handler;

fn main() {
    let run_cmd = SubCommand::with_name("run")
        .about("Runs the client")
        .arg(Arg::new("username")
            .short('u')
            .long("username")
            .takes_value(true)
            .value_name("USERNAME")
            .required(true)
            .help("The username used by the instance of the client."))
        .arg(Arg::new("server")
            .short('s')
            .long("server")
            .takes_value(true)
            .value_name("SERVER URL")
            .default_value("localhost:7878")
            .help("The server url for the games."));

    let app = App::new("La patata caliente")
        .about("A client app for 'La patate chaude'... but it's in a Spanish name.")
        .author("Paul Barrié | Paolo Manaois | Adem Mrizak")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(run_cmd);

    match app.get_matches().subcommand() {
        Some(("run", run_args)) => {
            if let (Some(username), Some(server)) = (
                run_args.get_one::<String>("username"), 
                run_args.get_one::<String>("server")
            ) {
                launch_client(username.to_string(), server.to_string());
            }
        },
        _ => unreachable!("Exhausted list of subcommands")
    }
}

fn launch_client(username: String, server: String) {
    println!("Connecting to server...\n");

    let stream = TcpStream::connect(server.as_str());
    let players: Vec<PublicPlayer> = vec![];
    let mut handle_message = handler::message_handler_builder(username, players);

    match stream {
        Ok(mut stream) => {
            // SEND MSG
            utils::write_message(&Message::Hello, &mut stream);
            
            loop {
                if !utils::read_message(&mut stream, &mut handle_message) {
                    break;
                }
            }
        },
        Err(e) => {
            println!("Cannot connect to server: {}", e);
        }
    }
}