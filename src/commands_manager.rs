use crate::commands::{help, play};
use crate::utils::game_start_errors::{self, GameStartErrors};

use crate::commands::help::help_command;
use crate::commands::play::play_command;

use crate::utils::coloured_print::coloured_print;
use crate::utils::colours::Colour;

pub fn getCommand(command: &str) -> bool {
    fn is_integer(s: &str) -> bool {
        s.parse::<i32>().is_ok()
    }

    let command_str = command.split_whitespace().collect::<Vec<&str>>();
    match command_str.get(0) {
        Some(&"help") => {
            help_command();
            false
        }
        Some(&"play") => {
            if command_str.len() != 4 {
                coloured_print(
                    &GameStartErrors::InvalidParameter.to_string(),
                    Colour::Red.rgb(),
                ); // Red
                false
            } else if command_str[1].is_empty()
                || command_str[2].is_empty()
                || command_str[3].is_empty()
            {
                coloured_print(
                    &GameStartErrors::EmptyParameter.to_string(),
                    Colour::Red.rgb(),
                ); // Orange
                false
            } else if !is_integer(command_str[1]) || !is_integer(command_str[3]) {
                coloured_print(
                    &GameStartErrors::InvalidParameter.to_string(),
                    Colour::Red.rgb(),
                ); // Red
                false
            } else {
                // Safe to unwrap here because we validated lengths and integers
                play_command(command_str[1], command_str[2], command_str[3]);
                false
            }
        }

        Some(&"quit") => {
            coloured_print("Quitting the game...", Colour::Blue.rgb()); // Cyan
            true
        }
        _ => {
            coloured_print("Invalid command", Colour::Red.rgb()); // Red
            false
        }
    }
}
