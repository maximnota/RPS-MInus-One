use rand::seq::SliceRandom;

/// Randomly choose 2 distinct objects
pub fn ai_choose_objects<'a>() -> Vec<&'a str> {
    let choices = ["rock", "paper", "scissors"];
    let mut rng = rand::thread_rng();

    // sample two distinct objects
    choices.choose_multiple(&mut rng, 2).cloned().collect()
}

/// Randomly choose 1 object from a given slice of &str
pub fn ai_choose_finale<'a>(objects: &'a [&'a str]) -> &'a str {
    let mut rng = rand::thread_rng();
    objects.choose(&mut rng).unwrap()
}
