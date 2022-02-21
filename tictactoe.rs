
enum Player {
    X,
    Y,
}

struct Board<'a> {
    board: &'a [Player],
}

fn main() {
    println!("Hello");
}