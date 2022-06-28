use serde::{Deserialize, Serialize};
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;
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

// impl MD5HashCashInput {
//     fn new(complexity: u32, message: &str) -> MD5HashCashInput {
//         MD5HashCashInput { complexity, message: String::from(message) }
//     }
// }

impl Challenge for MD5HashCashChallenge {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        todo!()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCashChallenge { input }
    }

    fn solve(&self) -> Self::Output {
        let mut checked: HashSet<u64> = HashSet::new();
        let mut rng = rand::thread_rng();
        let field = Uniform::from(0..u64::MAX);
    
        loop {
            let mut seed = field.sample(&mut rng);
            while checked.contains(&seed) {
                seed = field.sample(&mut rng);
            }
            checked.insert(seed);

            let input = format!("{seed:0>16X}{}", self.input.message);
            let hashcode = format!("{:X}", md5::compute(&input));
    
            let zeros = count_bits_to_zero(&hashcode);
            if zeros >= self.input.complexity {
                return Self::Output {
                    seed,
                    hashcode
                };
            }
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let seed = answer.seed;
        let input = format!("{seed:0>16X}{}", self.input.message);
        let hashcode = format!("{:X}", md5::compute(&input));
        answer.hashcode == hashcode
    }
}

fn count_bits_to_zero(hex_string: &str) -> u32 {
    let hex_value = u128::from_str_radix(hex_string, 16).unwrap();
    hex_value.leading_zeros()
}