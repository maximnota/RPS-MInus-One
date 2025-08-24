use std::io;

use crate::utils::coloured_print::*;
use crate::utils::colours::Colour;

use crate::engines::rps_1;
use crate::engines::rps_1::MarkovModel;
use crate::engines::rps_random;

use rand::seq::SliceRandom;

fn rps_result(a: &str, b: &str) -> i8 {
    match (a, b) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => 1, // win
        ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => -1, // loss
        _ => 0,                                                                // tie
    }
}

pub fn ai_choose_finale<'a>(enemy_choices: &[&'a str], my_choices: &[&'a str]) -> &'a str {
    let mut best_choice = my_choices[0];
    let mut best_score = i32::MIN;

    for &my in my_choices {
        let mut score = 0;

        for &enemy in enemy_choices {
            score += match rps_result(my, enemy) {
                1 => 1,   // win = +1
                -1 => -1, // loss = -1
                _ => 0,   // tie = 0
            };
        }

        if score > best_score {
            best_score = score;
            best_choice = my;
        }
    }

    best_choice
}

fn compare_logic(player_1: &str, player_2: &str) -> i8 {
    match (player_1, player_2) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => 1, // player wins
        ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => -1, // AI wins
        _ => 0,                                                                // tie
    }
}

fn expand_shorthand(input: &str) -> Option<String> {
    match input {
        "r" => Some("rock".to_string()),
        "p" => Some("paper".to_string()),
        "s" => Some("scissors".to_string()),
        "rock" | "paper" | "scissors" => Some(input.to_string()),
        _ => None,
    }
}

fn choose_objects() -> Option<Vec<String>> {
    print_colored_word(
        "Enter your chosen objects (2 words or shorthand like 'rp'): ",
        Colour::Blue.rgb(),
    );

    let _ = io::Write::flush(&mut io::stdout());
    let mut objects = String::new();

    io::stdin()
        .read_line(&mut objects)
        .expect("Failed to read line");

    let input = objects.trim();

    // If user typed combined shorthand (e.g. "rs"), split into chars
    let tokens: Vec<String> = if input.len() == 2 && !input.contains(' ') {
        input.chars().map(|c| c.to_string()).collect()
    } else {
        input.split_whitespace().map(String::from).collect()
    };

    // Expand each token
    let mut expanded = Vec::new();
    for token in tokens {
        if let Some(full) = expand_shorthand(&token) {
            expanded.push(full);
        } else {
            coloured_print(
                "Invalid input — use rock/paper/scissors or r/p/s",
                Colour::Red.rgb(),
            );
            return None;
        }
    }

    // Validate length
    if expanded.len() != 2 {
        if expanded.len() < 2 {
            coloured_print("Not enough objects", Colour::Red.rgb());
        } else {
            coloured_print("Too many objects", Colour::Red.rgb());
        }
        return None;
    }

    Some(expanded)
}

fn minus_one(objects: &[String]) -> Option<String> {
    print_colored_word("Which object do you keep (r/p/s): ", Colour::Blue.rgb());
    let _ = io::Write::flush(&mut io::stdout());
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim();

    // Expand shorthand if possible
    if let Some(expanded) = expand_shorthand(choice) {
        if objects.iter().any(|o| o == &expanded) {
            return Some(expanded);
        }
    }

    None
}

pub fn play_command(param1: &str, param2: &str, param3: &str) {
    // Parse max_turns safely
    let max_turns: usize = match param1.parse() {
        Ok(n) => n,
        Err(_) => {
            coloured_print("Invalid number of turns.", Colour::Red.rgb());
            return;
        }
    };

    let mut my_history: Vec<String> = Vec::new();
    let mut opp_history: Vec<String> = Vec::new();

    let mut turn = 0;
    let mut player_1_score = 0;
    let mut player_2_score = 0;

    // Create a MarkovModel once outside the loop
    let mut markov_model = MarkovModel::new();

    let winning_score: i32 = match param3.parse() {
        Ok(n) => n,
        Err(_) => {
            coloured_print("Invalid winning score.", Colour::Red.rgb());
            return;
        }
    };

    while turn < max_turns {
        if player_1_score >= winning_score || player_2_score >= winning_score {
            coloured_print("Game over!", Colour::Yellow.rgb());
            break;
        }

        // AI chooses its pair
        let ai_choices: Vec<String> = match param2 {
            "rps_random" => rps_random::ai_choose_objects()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            "rps_1" => rps_1::ai_choose_objects(turn, &opp_history, &mut markov_model),
            _ => {
                coloured_print("Invalid AI engine parameter.", Colour::Red.rgb());
                return;
            }
        };

        // Display current score
        coloured_print(
            &format!(
                "Current score: You: {}, AI: {}",
                player_1_score, player_2_score
            ),
            Colour::Green.rgb(),
        );

        // Player chooses 2 objects
        let objects: Vec<String> = loop {
            if let Some(objs) = choose_objects() {
                break objs;
            }
        };

        coloured_print(
            &format!("AI chose: {} {}", ai_choices[0], ai_choices[1]),
            Colour::Purple.rgb(),
        );

        // Player keeps 1 object
        let object = loop {
            if let Some(obj) = minus_one(&objects) {
                break obj;
            } else {
                coloured_print("Invalid choice, try again.", Colour::Red.rgb());
            }
        };
        coloured_print(&format!("You chose: {}", object), Colour::Yellow.rgb());

        // Prepare AI choices slices outside the match to avoid temporary value issues
        let ai_choices_slices: Vec<&str> = ai_choices.iter().map(|s| s.as_str()).collect();

        let final_choice: String = match param2 {
            "rps_random" => rps_random::ai_choose_finale(
                &ai_choices.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            )
            .to_string(), // convert &str -> String
            "rps_1" => rps_1::ai_choose_finale(
                &objects.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                &ai_choices.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            ),
            _ => {
                coloured_print("Invalid AI engine parameter.", Colour::Red.rgb());
                return;
            }
        };

        coloured_print(&format!("AI kept: {}", final_choice), Colour::Purple.rgb());

        // Update histories
        my_history.push(object.clone());
        opp_history.push(final_choice.to_string());

        // Compare results
        match compare_logic(&object, &final_choice) {
            1 => {
                coloured_print("You win!", Colour::Yellow.rgb());
                player_1_score += 1;
            }
            -1 => {
                coloured_print("AI wins!", Colour::Yellow.rgb());
                player_2_score += 1;
            }
            _ => {
                coloured_print("It's a tie!", Colour::Blue.rgb());
            }
        }

        turn += 1;
    }

    coloured_print(
        &format!(
            "Final score: You: {}, AI: {}",
            player_1_score, player_2_score
        ),
        Colour::Green.rgb(),
    );
}
