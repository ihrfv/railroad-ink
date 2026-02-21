use super::dice::{DICE_CONNECTION_ROUTES, DICE_REGULAR_ROUTES};
use super::route::RouteKind;
use std::io;

const GAME_ROUNDS: usize = 7;
const ROUTES_PER_ROUNDS: usize = 4;
const REGULAR_ROUTES_PER_ROUNDS: usize = 3;

pub struct Game {
    rounds: [RoundRoll; GAME_ROUNDS],
}

pub struct RoundRoll {
    routes: [RouteKind; ROUTES_PER_ROUNDS],
}

impl Game {
    pub fn present_rounds(&self) {
        for (round, roll) in self.rounds.iter().enumerate() {
            println!("--------------------");
            println!("Round {}", round + 1);
            println!("--------------------");
            roll.print();
            println!("--------------------");
            println!("Hit enter to reveal the next round");
            // waiting until enter is pressed
            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer).unwrap();
        }
        println!("--------------------");
        println!("The end of the game!");
    }

    pub fn generate_random() -> Game {
        Game {
            rounds: std::array::from_fn(|_| RoundRoll::generate_random()),
        }
    }
}

impl RoundRoll {
    fn print(&self) {
        for (dice, route_kind) in self.routes.iter().enumerate() {
            println!("Dice {}: {:?}", dice + 1, route_kind);
            print!("{}", route_kind.get_route().get_image_as_string())
        }
    }
    fn generate_random() -> RoundRoll {
        RoundRoll {
            routes: std::array::from_fn(|i| {
                if i < REGULAR_ROUTES_PER_ROUNDS {
                    DICE_REGULAR_ROUTES.roll()
                } else {
                    DICE_CONNECTION_ROUTES.roll()
                }
            }),
        }
    }
}
