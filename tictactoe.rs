// use std::fmt;

struct Board {
    board: [u8; 9],
    player: u8,
}

impl Board {
    fn new() -> Board {
        let mut newBoard = Board {
            board: [0;9], 
            player: 1,
        };
        return newBoard;
    }

    fn display(&self) {
        println!("__________________");
        println!("|     |     |     |");
        for i in 0..9 {
            print!("|  ");
            match self.board[i] {
                1 => print!("X"),
                2 => print!("O"),
                _ => print!("{}", i),
            }
            match i {
                2 => print!("  |\n|_____|_____|_____|\n|     |     |     |\n"),
                5 => print!("  |\n|_____|_____|_____|\n|     |     |     |\n"),
                8 => print!("  |\n|_____|_____|_____|\n"),
                _ => print!("  "),
            }
        }
    }

}

fn main() {
    let board: Board = Board::new();
    board.display();
}