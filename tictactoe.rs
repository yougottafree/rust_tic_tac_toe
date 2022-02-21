// use std::fmt;

struct Board {
    board: [u8; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            board: [0;9],
        }  
    }

    fn put(&mut self, index: usize, player:u8) -> bool {
        if self.board[index] != 0 {
            println!("This is already occupied");
            return false;
        }
        self.board[index] = player;
        return true;
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
    let mut board: Board = Board::new();
    board.display();
    let index = 1 as usize;
    board.put(index, 1);
    board.display();
    board.put(2 as usize, 2);
    board.display();
}