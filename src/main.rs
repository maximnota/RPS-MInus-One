// main.rs

mod commands;
mod commands_manager;
mod engines;
mod utils;

use crate::utils::coloured_print::coloured_print;
use crate::utils::colours::Colour;

fn main() {
    coloured_print(
        r#"____________________  _________            _____  .__                              ________
\______   \______   \/   _____/           /     \ |__| ____  __ __  ______         \_____  \   ____   ____
 |       _/|     ___/\_____  \   ______  /  \ /  \|  |/    \|  |  \/  ___/  ______  /   |   \ /    \_/ __ \
 |    |   \|    |    /        \ /_____/ /    Y    \  |   |  \  |  /\___ \  /_____/ /    |    \   |  \  ___/
 |____|_  /|____|   /_______  /         \____|__  /__|___|  /____//____  >         \_______  /___|  /\___  >
        \/                  \/                  \/        \/           \/                  \/     \/     \/ "#,
        Colour::Yellow.rgb(),
    );
    coloured_print(
        "Welcome to rock, paper, scissors minus one. For help regarding commands type: help.",
        Colour::Red.rgb(),
    );
    loop {
        let mut user_command = String::new();
        std::io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to read line");

        let user_command = user_command.trim().to_lowercase();

        let flag = commands_manager::getCommand(&user_command);
        if flag {
            break;
        }
    }
}
