use serde::{Deserialize, Serialize};
use crate::challenge::Challenge;

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashChallenge {
    input: MD5HashCashInput
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

impl Challenge for MD5HashCashChallenge {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCashChallenge { input }
    }

    fn solve(&self) -> Self::Output {
        for seed in 0..=u64::MAX {
            let input = format!("{seed:0>16X}{}", self.input.message);
            let num_hashcode = u128::from_ne_bytes(md5::compute(&input).0);
            let hashcode = format!("{:X}", num_hashcode);
    
            let zeros = num_hashcode.leading_zeros();
            if zeros >= self.input.complexity {
                return Self::Output {
                    seed,
                    hashcode
                };
            }
        }

        Self::Output {
            seed: 0,
            hashcode: "".to_string()
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let seed = answer.seed;
        let input = format!("{seed:0>16X}{}", self.input.message);
        let hashcode = format!("{:X}", md5::compute(&input));
        count_bits_to_zero(&hashcode) >= self.input.complexity && answer.hashcode == hashcode
    }
}

fn count_bits_to_zero(hex_string: &str) -> u32 {
    let hex_value = u128::from_str_radix(hex_string, 16).unwrap();
    hex_value.leading_zeros()
}