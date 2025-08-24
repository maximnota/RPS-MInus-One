use crate::utils::coloured_print::{self, *};
use crate::utils::colours::Colour;

pub fn help_command() {
    coloured_print("\n Commands:", Colour::Green.rgb());
    print_colored_word("    -play ", Colour::Blue.rgb());
    print_colored_word("<max_wins: int> ", Colour::Purple.rgb());
    print_colored_word("<engine_selection: str> ", Colour::Orange.rgb());
    print_colored_word("<turns_amount: int> \n", Colour::Purple.rgb());

    // println!("  help");
    print_colored_word("    -help ", Colour::Blue.rgb());
    print_colored_word("for printing this information \n", Colour::Yellow.rgb());
}
