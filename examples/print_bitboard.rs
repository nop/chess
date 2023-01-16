//! Print the bitboard with given `Square`s set.
use std::env;
use std::str::FromStr;
use chess::repr::board::BitBoard;
use chess::repr::board::square::Square;

fn main() {
    let args = env::args();
    let mut board = BitBoard::default();
    for arg in args.skip(1) {
        let sq: Square = Square::from_str(&arg).unwrap();
        board.set(sq.into());
    }
    // println!("{:?}", env::args().skip(1).collect::<Vec<_>>());
    println!("{board}");
}