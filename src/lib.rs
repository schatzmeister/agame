use rand::seq::SliceRandom;
use std::io;

/// Struct holding the complete data for one game.
#[derive(Debug, Default, PartialEq, Eq)]
struct Game {
    deck: Vec<u8>,
    hand: Vec<u8>,
    // The stacks of cards
    up1: Vec<u8>,
    up2: Vec<u8>,
    down1: Vec<u8>,
    down2: Vec<u8>,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,
            "({decklen})|{up1} {up2} {down1} {down2}|{hand:?}",
            decklen = self.deck.len(),
            up1 = match self.up1.last() {
                Some(x) => x.to_string(),
                None => "X".to_string(),
            },
            up2 = match self.up2.last() {
                Some(x) => x.to_string(),
                None => "X".to_string(),
            },
            down1 = match self.down1.last() {
                Some(x) => x.to_string(),
                None => "X".to_string(),
            },
            down2 = match self.down2.last() {
                Some(x) => x.to_string(),
                None => "X".to_string(),
            },
            hand = self.hand,
        )
    }
}

impl Game {
    /// Create a new instance of Game.
    fn new() -> Self {
        let mut deck: Vec<_> = (1..=99).collect();
        deck.shuffle(&mut rand::thread_rng());
        Self {
            deck,
            ..Game::default()
        }
    }

    /// Draw the topmost  amount  cards from the deck.
    fn draw(&mut self, amount: u8) {
        self.hand
            .extend(self.deck.split_off(self.deck.len() - (amount as usize)));
    }
}

/// The REPL wrapper around the game.
///
/// manages in- and output.
pub fn repl() {
    startup();

    loop {
        let input = {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => input,
                Err(e) => panic!("Error occurred: {}", e),
            }
        };
        if input == "exit\n" {
            break;
        } else {
            println!("Status: {}", input);
        }
    }
}

/// REPL statup function to initialize the game.
fn startup() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("Welcome to a game v{}", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_test() {
        let mut game = Game::new();
        game.draw(8);
        assert_eq!(game.hand.len(), 8);
    }
}
