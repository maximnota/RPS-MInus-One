use itertools::Itertools;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MarkovModel {
    table: HashMap<(String, String), HashMap<String, usize>>,
}

impl MarkovModel {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn update(&mut self, prev: (String, String), next: &str) {
        let entry = self.table.entry(prev).or_insert_with(HashMap::new);
        *entry.entry(next.to_string()).or_insert(0) += 1;
    }

    pub fn predict(&self, prev: (String, String)) -> Option<(String, f64)> {
        self.table.get(&prev).and_then(|counts| {
            let total: usize = counts.values().sum();
            if total == 0 {
                return None;
            }
            let (best_move, best_count) = counts.iter().max_by_key(|(_, c)| *c)?;
            let confidence = *best_count as f64 / total as f64;
            Some((best_move.clone(), confidence))
        })
    }
}

fn rps_result(a: &str, b: &str) -> i8 {
    match (a, b) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => -1, // AI wins
        ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => 1,  // player wins
        _ => 0,                                                                 // tie
    }
}
pub fn ai_choose_finale(player_choices: &[&str], ai_choices: &[&str]) -> String {
    let mut rng = rand::thread_rng();

    // Separate moves into categories
    let mut wins = Vec::new();
    let mut ties = Vec::new();
    let mut losses = Vec::new();

    for &ai in ai_choices {
        let mut total = 0;
        for &player in player_choices {
            total += rps_result(player, ai);
        }

        if total > 0 {
            wins.push(ai);
        } else if total == 0 {
            ties.push(ai);
        } else {
            losses.push(ai);
        }
    }

    // Pick in order: win > tie > loss
    if !wins.is_empty() {
        return wins[rng.gen_range(0..wins.len())].to_string();
    } else if !ties.is_empty() {
        return ties[rng.gen_range(0..ties.len())].to_string();
    } else {
        return losses[rng.gen_range(0..losses.len())].to_string();
    }
}

fn counter_move(predicted: &str) -> String {
    match predicted {
        "rock" => "paper".to_string(),
        "paper" => "scissors".to_string(),
        "scissors" => "rock".to_string(),
        _ => "rock".to_string(),
    }
}

pub fn ai_choose_objects(
    turn: usize,
    opp_moves: &[String],
    model: &mut MarkovModel,
) -> Vec<String> {
    let all_pairs: Vec<Vec<String>> = vec![
        vec!["paper".to_string(), "rock".to_string()],
        vec!["scissors".to_string(), "paper".to_string()],
        vec!["scissors".to_string(), "rock".to_string()],
    ];

    let mut scores: HashMap<Vec<String>, f64> =
        all_pairs.iter().cloned().map(|pair| (pair, 0.0)).collect();

    let score_keys: Vec<Vec<String>> = scores.keys().cloned().collect();

    if turn >= 2 {
        let prev = (opp_moves[turn - 2].clone(), opp_moves[turn - 1].clone());
        if let Some((predicted, confidence)) = model.predict(prev) {
            let counter = counter_move(&predicted);
            for pair in &score_keys {
                if pair.contains(&counter) {
                    *scores.get_mut(pair).unwrap() += confidence;
                }
            }
        }
    }

    if turn >= 3 {
        let last3 = &opp_moves[turn - 3..turn];
        if last3[0] == last3[2] && last3[0] != last3[1] {
            let counter = counter_move(&last3[2]);
            for pair in &score_keys {
                if pair.contains(&counter) {
                    *scores.get_mut(pair).unwrap() += 0.5;
                }
            }
        }
    }

    let mut rng = rand::thread_rng();
    for value in scores.values_mut() {
        *value += rng.gen_range(0.0..0.1); // adds slight randomness
    }

    // Return the best pair or fallback to first
    scores
        .into_iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(pair, _)| pair)
        .unwrap_or_else(|| all_pairs[0].clone())
}
