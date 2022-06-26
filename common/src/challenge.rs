use serde::{Deserialize, Serialize};

pub trait Challenge {
    type Input;
    type Output;

    fn name() -> String;
    fn new(input: Self::Input) -> Self;
    fn solve(&self) -> Self::Output;
    fn verify(&self, answer: &Self::Output) -> bool;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashChallenge {
    input: MD5HashCashInput
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MD5HashCashInput {
    complexity: u32,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

impl MD5HashCashInput {
    fn new(complexity: u32, message: &str) -> MD5HashCashInput {
        MD5HashCashInput { complexity, message: String::from(message) }
    }
}

impl Challenge for MD5HashCashChallenge {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        todo!()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCashChallenge { input }
    }

    fn solve(&self) -> Self::Output { // TODO
        Self::Output {
            seed: 0x034C,
            hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3")
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        true
    }
}