use super::dice::{DICE_CONNECTION_ROUTES, DICE_REGULAR_ROUTES};
use super::route::RouteKind;
use std::io;

pub struct Game {
    rounds: [RoundRoll; 7],
}

pub struct RoundRoll {
    routes: [RouteKind; 4],
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
    }

    pub fn generate_random() -> Game {
        Game {
            rounds: [
                RoundRoll::generate_random(),
                RoundRoll::generate_random(),
                RoundRoll::generate_random(),
                RoundRoll::generate_random(),
                RoundRoll::generate_random(),
                RoundRoll::generate_random(),
                RoundRoll::generate_random(),
            ],
        }
    }
}

impl RoundRoll {
    fn print(&self) {
        for (dice, route_kind) in self.routes.iter().enumerate() {
            println!("Dice {}: {:?}", dice + 1, route_kind);
        }
    }
    fn generate_random() -> RoundRoll {
        RoundRoll {
            routes: [
                DICE_REGULAR_ROUTES.roll(),
                DICE_REGULAR_ROUTES.roll(),
                DICE_REGULAR_ROUTES.roll(),
                DICE_CONNECTION_ROUTES.roll(),
            ],
        }
    }
}
