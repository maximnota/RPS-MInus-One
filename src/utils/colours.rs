// Keep RGB enum for type safety
pub enum RGB {
    Red(u8),
    Green(u8),
    Blue(u8),
}

#[derive(Debug)]
pub enum Colour {
    Orange,
    Purple,
    Yellow,
    Green,
    Blue,
    Red,
}

impl Colour {
    pub fn rgb(&self) -> (u8, u8, u8) {
        // Each colour is defined as a tuple of RGB enums
        let channels = match self {
            Colour::Orange => (RGB::Red(255), RGB::Green(165), RGB::Blue(0)),
            Colour::Purple => (RGB::Red(128), RGB::Green(0), RGB::Blue(128)),
            Colour::Yellow => (RGB::Red(255), RGB::Green(255), RGB::Blue(0)),
            Colour::Green => (RGB::Red(0), RGB::Green(255), RGB::Blue(0)),
            Colour::Blue => (RGB::Red(0), RGB::Green(0), RGB::Blue(255)),
            Colour::Red => (RGB::Red(255), RGB::Green(0), RGB::Blue(0)),
        };

        // Convert the RGB enums into a tuple
        let red = if let RGB::Red(val) = channels.0 {
            val
        } else {
            0
        };
        let green = if let RGB::Green(val) = channels.1 {
            val
        } else {
            0
        };
        let blue = if let RGB::Blue(val) = channels.2 {
            val
        } else {
            0
        };

        (red, green, blue)
    }
}
