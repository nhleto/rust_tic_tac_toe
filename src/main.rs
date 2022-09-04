use std::{io};
mod domain;
use array2d::Array2D;
use domain::Domain::{self, Symbol, Game};

fn main() {
    let mut game = Game {
        player1: Domain::Player { name: String::new(), symbol: Domain::Symbol::X },
        player2: Domain::Player { name: String::new(), symbol: Domain::Symbol::O },
        board: Domain::Board {},
        answers: [String::from("1"), String::from("2"), String::from("3"),String::from("4"), String::from("5"), String::from("6"),String::from("7"), String::from("8"), String::from("9")],
        current_player: &mut Domain::Player { name: String::from(""), symbol: Symbol::O },
        game_over: false
    };

    game.start_game();
}

impl<'a> Game<'a> {
    fn start_game(&mut self) {
        self.assign_user_names();
        println!("{:?}, {:?}", self.player1, self.player2);
        self.game_loop();
    }

    fn game_loop(&mut self) {
        let mut game_over = false;
        while !game_over {
            self.board.draw_board(&self.answers);
            self.get_guess();
            game_over = self.check_answer();
        }
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
        self.parse_guess();
        self.board.draw_board(&self.answers);
    }

    fn parse_guess(&mut self) {
        loop {
            let mut input_buffer = String::new();

            io::stdin()
                .read_line(&mut input_buffer)
                .expect("Failed to read input!");
                
            match input_buffer.trim().parse::<usize>() {
                Ok(result) => if self.validate_guess_against_board(result) {
                    break
                },
                Err(er) => println!("Invald input: {}", &er),
            };
        }
    }

    fn validate_guess_against_board(&mut self, guess: usize) -> bool {
        let adjusted_guess = guess - 1;
        match self.answers[adjusted_guess] != Symbol::X.to_string() && self.answers[adjusted_guess] != Symbol::O.to_string() {
            true => {
                self.answers[adjusted_guess] = self.current_player.symbol.to_string();
                true
            },
            false => {
                println!("That spot is already taken");
                false
            }
        }
    }

    fn check_answer(&mut self) -> bool {
        let two_d_array = &mut Array2D::from_row_major(&self.answers.to_vec(), 3, 3);
        let mut right_diagonal = 0;
        let mut left_diagonal = 0;

        // Solve diagonal
        for (i, elem) in two_d_array.as_rows().iter().enumerate() {
            if i == 0 && (elem[0] == Symbol::X.to_string() || elem[0] == Symbol::O.to_string()) {
                right_diagonal += 1;
            }

            if i == 1 && (elem[1] == Symbol::X.to_string() || elem[1] == Symbol::O.to_string()) {
                right_diagonal += 1;
            }

            if i == 2 && (elem[elem.len() - 1] == Symbol::X.to_string() || elem[elem.len() - 1] == Symbol::O.to_string()) {
                right_diagonal += 1;
            }

            if right_diagonal == 3 {
                return true
            }
        }

        // Solve inverse diagonal
        for (i, elem) in two_d_array.as_rows().iter().enumerate() {
            if i == 0 && elem[elem.len() - 1] == Symbol::X.to_string() || elem[elem.len() - 1] == Symbol::O.to_string() {
                left_diagonal += 1;
            }

            if i == 1 && (elem[1] == Symbol::X.to_string() || elem[1] == Symbol::O.to_string()) {
                left_diagonal += 1;
            }

            if i == 2 && (elem[0] == Symbol::X.to_string() || elem[0] == Symbol::O.to_string()) {
                left_diagonal += 1;
            }

            if left_diagonal == 3 {
                return true
            }
        }
        
        // Solve horizontal
        for mut row_iter in two_d_array.rows_iter() {
            if row_iter.all(|elem| elem.to_string() == Symbol::X.to_string() || elem.to_string() == Symbol::O.to_string()) {
                return true
            }
        }

        // Solve vertical
        for mut columns_iter in two_d_array.columns_iter() {
            if columns_iter.all(|elem| elem.to_string() == Symbol::X.to_string() || elem.to_string() == Symbol::O.to_string()) {
                return true
            }
        }

        false
    }
    
    // fn get_player(&mut self) {
    //     self.current_player = match self.current_player.symbol {
    //         Symbol::X => &self.player1,
    //         Symbol::O => &self.player2,
    //     };

    //     println!("The current player is: {:?}", self.current_player)
    // }
}
