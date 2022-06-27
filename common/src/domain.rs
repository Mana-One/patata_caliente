use serde::{Deserialize, Serialize};
use crate::challenge::md5_hashcash::MD5HashCashOutput;

#[derive(Deserialize, Serialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered, 
    InvalidName
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64
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