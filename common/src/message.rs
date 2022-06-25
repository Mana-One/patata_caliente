use serde::{Deserialize, Serialize};
use crate::domain::{
    ChallengeAnswer, 
    PublicPlayer, 
    ReportedChallengeResult, 
    SubscribeError
};

#[derive(Deserialize, Serialize)]
pub enum Message {
    Hello
}

#[derive(Deserialize, Serialize)]
pub struct Welcome {
    version: u8
}

#[derive(Deserialize, Serialize)]
pub struct Subscribe {
    name: String
}

#[derive(Deserialize, Serialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Deserialize, Serialize)]
pub struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Deserialize, Serialize)]
pub enum Challenge {
    // ChallengeName()
}

#[derive(Deserialize, Serialize)]
pub struct ChallengeResult {
    name: ChallengeAnswer,
    next_target: String
}

#[derive(Deserialize, Serialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Deserialize, Serialize)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard
}