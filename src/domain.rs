pub mod Domain {
    use strum_macros::{EnumString, Display};

    #[derive(Debug)]
    pub struct Game<'a> {
        pub player1: Player,
        pub player2: Player,
        pub board: Board,
        pub answers: [String; 9],
        pub current_player: &'a Player
    }
    
    #[derive(Debug)]
    pub struct Player {
        pub name: String,
        pub symbol: Symbol
    }

    #[derive(Debug, EnumString, Clone, Copy, Display)]
    pub enum Symbol {
        X,
        O
    }

    #[derive(Debug)]
    pub struct Board;

    impl Board {
        pub fn draw_board(&self, answers: &[String]) {
            println!("{}|{}|{}", answers[0], answers[1], answers[2]);
            println!("-----");
            println!("{}|{}|{}", answers[3], answers[4], answers[5]);
            println!("-----");
            println!("{}|{}|{}", answers[6], answers[7], answers[8]);
        }    
    }
}