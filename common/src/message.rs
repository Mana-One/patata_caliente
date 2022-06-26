use serde::{Deserialize, Serialize};
use crate::domain::{
    ChallengeAnswer, 
    PublicPlayer, 
    ReportedChallengeResult, 
    SubscribeError
};
use crate::challenge:: MD5HashCashInput;

#[derive(Deserialize, Serialize, Debug)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Welcome {
    version: u8
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Subscribe {
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Deserialize, Serialize, Debug)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChallengeResult {
    result: ChallengeAnswer,
    next_target: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard
}

impl Subscribe {
    pub fn new(name: &str) -> Subscribe {
        Subscribe { name: String::from(name) }
    }
}

impl ChallengeResult {
    pub fn new(result: ChallengeAnswer, next_target: &str) -> ChallengeResult {
        ChallengeResult { result, next_target: String::from(next_target) }
    }
}