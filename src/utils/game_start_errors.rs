use std::fmt;

#[derive(Debug)]
pub enum GameStartErrors {
    AlreadyActive,
    InvalidParameter,
    EmptyParameter,
    InvalidEngine,
}

impl GameStartErrors {
    // Returns the error message for each variant
    pub fn message(&self) -> &'static str {
        match self {
            GameStartErrors::AlreadyActive => {
                "Game is already active. You can't start a new game while one is already running."
            }
            GameStartErrors::EmptyParameter => {
                "Empty parameter: You left a needed parameter empty. For help use the help command"
            }
            GameStartErrors::InvalidParameter => {
                "Invalid parameter: please provide the correct parameter. For help use the help command"
            }
            GameStartErrors::InvalidEngine => {
                "Invalid engine - Current engines implemented: [rpsmo1]"
            }
        }
    }
}

impl fmt::Display for GameStartErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
