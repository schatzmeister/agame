use rand::seq::SliceRandom;
use std::io;

/// Game related error.
#[derive(std::fmt::Debug, PartialEq, Eq)]
struct GameError(String);

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for GameError {}

/// Struct holding the complete data for one game.
#[derive(Debug, Default, PartialEq, Eq)]
struct Game {
    deck: Vec<u8>,
    hand: Vec<u8>,
    // The piles of cards to play on
    up1: Vec<u8>,
    up2: Vec<u8>,
    down1: Vec<u8>,
    down2: Vec<u8>,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
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

    /// Play a card from the hand to a specific pile.
    fn play(&mut self, card: &u8, pile: &str) -> Result<(), GameError> {
        // Check if the pile is correct.
        let pile = match pile {
            "up1" => &mut self.up1,
            "up2" => &mut self.up2,
            "down1" => &mut self.down1,
            "down2" => &mut self.down2,
            s => return Err(GameError(format!("Pile `{}` does not exist.", s)))
        };
        // Check if the card exist in the hand.
        match self.hand.iter().position(|x| x == card) {
            Some(index) => {
                pile.push(self.hand.remove(index));
                Ok(())
            },
            None => Err(GameError(format!("Card `{}` not available.", card))),
        }
        // TODO: Check if the card is actually playable.
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

    #[test]
    fn play_test() {
        let mut game = Game::new();
        game.draw(8);
        let card = game.hand.last().unwrap().to_owned();
        game.play(&card, "up1").unwrap();
        assert_eq!(game.up1.len(), 1);
        assert_eq!(game.hand.len(), 7);
    }
}
