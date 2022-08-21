use std::{io};
mod domain;
use domain::Domain::{self, Symbol, Game};

fn main() {
    let mut game = Game {
        player1: Domain::Player { name: String::new(), symbol: Domain::Symbol::X },
        player2: Domain::Player { name: String::new(), symbol: Domain::Symbol::O },
        board: Domain::Board {},
        answers: [String::from("1"), String::from("2"), String::from("3"),String::from("4"), String::from("5"), String::from("6"),String::from("7"), String::from("8"), String::from("9")],
        current_player: &mut Domain::Player { name: String::from(""), symbol: Symbol::O }
    };

    game.start_game();
}

impl<'a> Game<'a> {
    fn start_game(&mut self) {
        self.assign_user_names();
        println!("{:?}, {:?}", self.player1, self.player2);
        self.board.draw_board(&self.answers);
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

    fn get_guess(&mut self) {
        loop {
            println!("{}, make your move", &self.player1.name);
            let guess = self.parse_guess();
            self.validate_guess_against_board(guess as usize);
            self.board.draw_board(&self.answers);
        }
    }

    fn parse_guess(&mut self) -> u8 {
        loop {
            let mut input_buffer = String::new();

            io::stdin()
                .read_line(&mut input_buffer)
                .expect("Failed to read input!");
                
             match input_buffer.trim().parse::<u8>() {
                Ok(result) => return result,
                Err(er) => println!("error while parsing {}", er),
            };
        }
    }

    fn validate_guess_against_board(&mut self, guess: usize) {
        match self.answers[guess] != Symbol::X.to_string() || self.answers[guess] != Symbol::O.to_string() {
            true => {
                self.answers[guess] = Symbol::X.to_string();
            },
            false => print!("Try again"),
        }
    }

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }
    
    // fn get_player(&mut self) {
    //     self.current_player = match self.current_player.symbol {
    //         Symbol::X => &self.player1,
    //         Symbol::O => &self.player2,
    //     };

    //     println!("The current player is: {:?}", self.current_player)
    // }
}
