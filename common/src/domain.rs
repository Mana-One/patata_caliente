use serde::{Deserialize, Serialize};
use crate::challenge::MD5HashCashOutput;

#[derive(Deserialize, Serialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered, 
    InvalidName
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}