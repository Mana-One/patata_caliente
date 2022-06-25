use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum SubscribeError {
    AlreadyRegistered, 
    InvalidName
}

#[derive(Deserialize, Serialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Deserialize, Serialize)]
pub enum ChallengeAnswer {
    // ChallengeName(ChallengeOutput)
}

#[derive(Deserialize, Serialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Deserialize, Serialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}