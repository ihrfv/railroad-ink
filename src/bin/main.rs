use railroad_ink::game::Game;

pub fn main() {
    println!("Railroad Ink");
    println!("Generated random game:");
    Game::generate_random().present_rounds();
}
