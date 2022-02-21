// use std::fmt;

struct Board {
    board: [u8; 9],
    cur_player: u8,
    prev_player: u8,
    moves: u8,
}

impl Board {
    fn new() -> Board {
        Board {
            board: [0;9],
            cur_player: 1u8,
            prev_player: 2u8,
            moves: 0u8,
        }  
    }

    fn change_player(&mut self) {
        if self.cur_player == 1 {
            self.cur_player = 2;
            self.prev_player = 1
        } else {
            self.cur_player = 1;
            self.prev_player = 2;
        }
    }

    fn put(&mut self, index: usize) {
        if self.board[index] != 0 {
            println!("This is already occupied");
        } else {
            self.board[index] = self.cur_player;
            self.change_player();
            self.moves+=1;
        }
    }

    fn get_current_player(&mut self) -> char {
        match self.cur_player {
            1 => 'X',
            2 => 'O',
            _ => ' ',
        }
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
    board.put(index);
    board.display();
    board.put(2 as usize);
    board.display();
    println!("Current {}", board.get_current_player());
}