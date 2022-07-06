use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};
use common::domain::PublicPlayer;

pub fn select_target(players: &Vec<PublicPlayer>) -> Option<String> {
    let last_player = players
        .iter()
        .min_by(|p1, p2| p1.score.cmp(&p2.score))?;

    if last_player.score == 0 { target_rand(players) }
        else { target_best(players )}
}

fn target_rand(players: &Vec<PublicPlayer>) -> Option<String> {
    let mut rng = thread_rng();
    let uniform = Uniform::from(0..players.len());
    let idx = uniform.sample(&mut rng);
    players.get(idx).map(|p| p.clone().name)
}

fn target_best(players: &Vec<PublicPlayer>) -> Option<String> {
    players
        .iter()
        .max_by(|p1, p2| p1.score.cmp(&p2.score))
        .map(|p| p.clone().name)
}