use std::{io};
mod domain;
use domain::Domain::{self, Symbol};

fn main() {
    let mut game = Domain::Game {
        player1: Domain::Player { name: String::new(), symbol: Domain::Symbol::X },
        player2: Domain::Player { name: String::new(), symbol: Domain::Symbol::O },
        board: Domain::Board {},
        answers: [String::from("1"), String::from("2"), String::from("3"),String::from("4"), String::from("5"), String::from("6"),String::from("7"), String::from("8"), String::from("9")],
        currentPlayer: Domain::Player { name: String::new(), symbol: Symbol::O }
    };

    game.start_game();
}

impl Domain::Game {
    fn start_game(&mut self) {
        self.assign_user_names();
        println!("{:?}, {:?}", self.player1, self.player2);
        self.board.draw_board(&self.answers);
        self.get_player();
        self.get_guess();
    }

    fn assign_user_names(&mut self) {
        println!("Player 1, please enter your name");
        let mut input = self.get_user_names();
        self.player1.name = String::from(input.trim());
        
        println!("Player 2, please enter your name");
        input = self.get_user_names();
        self.player2.name = String::from(input.trim());
    }

    fn get_user_names(&mut self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to readline");
        input
    }

    fn get_guess(&mut self) -> u8 {
        println!("{}, make your move", &self.player1.name);
        Domain::Game::parse_guess()
    }

    fn parse_guess() -> u8 {
        loop {
            let mut input_buffer = String::new();

            io::stdin()
                .read_line(&mut input_buffer)
                .expect("Failed to read input!");
                
            match input_buffer.trim().parse::<u8>() {
                Ok(result) => return result,
                Err(er) => println!("Invalid input. Error: {}", er),
            }
        }
    }

    fn get_player(&mut self) {
        self.currentPlayer = match self.currentPlayer.symbol {
            Symbol::X => self.player1,
            Symbol::O => self.player2,
        };

        println!("The current player is: {:?}", self.currentPlayer)
    }

    fn validate_guess_against_board(&self, guess: u8) {
        // if self.answers[guess - 1] != 
    }

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }
}
