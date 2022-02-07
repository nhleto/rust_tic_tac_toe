use std::{io};
mod domain;
use domain::Domain;

fn main() {
    let mut game = Domain::Game {
        player1: Domain::Player { name: String::new(), symbol: String::from("O") },
        player2: Domain::Player { name: String::new(), symbol: String::from("X") },
        board: Domain::Board {},
        answers: [String::new(), String::new(), String::new(),String::new(), String::new(), String::new(),String::new(), String::new(), String::new()]
    };

    game.start_game();
}

impl Domain::Game {
    fn start_game(&mut self) {
        self.assign_user_names();

        println!("{:?}, {:?}", self.player1, self.player2);

        self.board.draw_board(&self.answers);
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
        println!("{}, make your move", &self.player1.name);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to readline");

        // match self.answers.get(guess) {
        //     Some(guess) => {
        //         if (self.answers[guess] != String::new())
        //     },
        //     None => {},
        // }
    }
}
