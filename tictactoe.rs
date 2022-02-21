use std::io;

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

    fn get_prev_player(&mut self) -> char {
        match self.prev_player {
            1 => 'X',
            2 => '0',
            _ => ' ',
        }
    }

    fn is_game_over(&mut self) -> bool {
        self.moves == 9 || self.get_winner() != ' '
    }

    fn get_winner(&mut self)-> char{
        if self.is_winner(self.cur_player) {
            return self.get_current_player();
        }
        if self.is_winner(self.prev_player) {
            return self.get_prev_player();
        }
        return ' ';
    }

    fn is_winner(&mut self, player:u8) -> bool {
        return self.check_row(player) || self.check_col(player) || self.check_diag(player);
    }

    fn check_row(&mut self, player:u8) -> bool {
        for row in 0..3 {
            let mut count:u8 = 0;
            for col in 0..3 {
                if self.board[row*3 + col] == player {
                    count += 1;
                }
            }
            if count == 3 {
                return true;
            }
        }
        return false;
    }

    fn check_col(&mut self, player:u8) -> bool {
        for col in 0..3 {
            let mut count:u8 = 0;
            for row in 0..3 {
                if self.board[row*3 + col] == player {
                    count += 1;
                }
            }
            if count == 3 {
                return true;
            }
        }
        return false;
    }

    fn check_diag(&mut self, player:u8) -> bool {
        if self.board[0] == player && self.board[4] == player && self.board[8] == player {
            return true;
        }
        if self.board[2] == player && self.board[4] == player && self.board[6] == player {
            return true;
        }
        return false;
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

fn play_new_game() {
    let mut board: Board = Board::new();
    while !board.is_game_over() {
        let mut number = String::new();
        board.display();
        println!("{} play at ", board.get_current_player());
        io::stdin().read_line(&mut number).expect("Failed to read input");
        let index:u8 = number.trim().parse().expect("invalid input");
        board.put(index as usize);
    }
    board.display();
    let winner:char = board.get_winner();
    if winner == ' ' {
        println!("DRAW!!");
    }
    println!("{} WINS!!", winner);
}


fn main() {
    let mut terminate:bool = false;
    while !terminate {
        println!("Please choose your option:");
        println!("n - play a new tictactoe game");
        println!("q - quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim() {
            "n" => play_new_game(),
            "q" => {terminate = true}, 
            _ => println!("Please input n or q"),
        }
    }
}