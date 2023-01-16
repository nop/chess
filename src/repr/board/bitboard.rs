//! Utilities for representing a bitboard.
//!
//! A [BitBoard] is a set of 64 bits that represents each square on the chess board.
//! A complete board representation contains a bitboard for each [piece] type and [color].
//! Bitboards model occupancy. That is, a 1 bit represents the presence of a piece on its bitboard,
//! where 0 represents the absence of that piece.
//!
//! ```plaintext
//! 8 . . . . . . . .
//! 7 . . . . . . . .
//! 6 . . . . . . . .
//! 5 . . . . . . . .
//! 4 . . . . . . . .
//! 3 . . . . . . . .
//! 2 . . . . . . . .
//! 1 . . . . . . . .
//!   a b c d e f g h
//! ```
//! [color]: https://en.wikipedia.org/wiki/Glossary_of_chess#color
//! [piece]: https://en.wikipedia.org/wiki/Glossary_of_chess#piece
use std::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not, Shl};

/// A `BitBoard` represents occupied and vacant positions on an 8x8 grid.
///
/// # Examples
/// ```
/// use chess::repr::board::BitBoard;
/// use chess::repr::board::square::Square;
/// let moves = vec![ "d4", "d5", "c4" ].into_iter().map(|mov| mov.parse::<Square>().unwrap());
/// let mut queens_gambit = BitBoard::default();
/// moves.for_each(|mov| queens_gambit.set(mov.into()));
/// assert_eq!(queens_gambit.population_count(), 3);
/// println!("{queens_gambit}")
/// ```
///
/// # Details
///
/// This BitBoard implementation uses a little-endian, file-rank mapping.
/// The index of each bit in the `BitBoard`'s internal `u64` corresponds to the board location
/// as follows:
///
/// ```plaintext
/// 8 | 56 57 58 59 60 61 62 63
/// 7 | 48 49 50 51 52 53 54 55
/// 6 | 40 41 42 43 44 45 46 47
/// 5 | 32 33 34 35 36 37 38 39
/// 4 | 24 25 26 27 28 29 30 31
/// 3 | 16 17 18 19 20 21 22 23
/// 2 | 08 09 10 11 12 13 14 15
/// 1 | 00 01 02 03 04 05 06 07
///     -----------------------
///      a  b  c  d  e  f  g  h
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Default, Hash)]
#[repr(transparent)]
pub struct BitBoard(u64);

impl BitBoard {
    /// The number of occupied squares on the `BitBoard`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::repr::board::BitBoard;
    /// use chess::repr::board::square::{File, Rank, Square};
    /// let e4 = Square{ file: File::E, rank: Rank::Four};
    /// let board = BitBoard::default().with(e4.into());
    /// assert_eq!(board.population_count(), 1);
    /// println!("{board}");
    /// ```
    pub fn population_count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Sets a bit in the BitBoard.
    pub fn set(&mut self, idx: u8) {
        self.0 |= 1 << idx;
    }

    /// Unsets a bit in the BitBoard.
    pub fn unset(&mut self, idx: u8) {
        self.0 &= !(1 << idx)
    }

    /// Returns a copied BitBoard with the given bit set.
    pub fn with(&self, idx: u8) -> BitBoard {
        let mut board = *self;
        board.set(idx);
        board
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Shl for BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn shl(self, rhs: Self) -> Self::Output {
        BitBoard((self.0).wrapping_shl(rhs.0 as u32))
    }
}

impl Shl<i64> for BitBoard {
    type Output = BitBoard;

    #[inline(always)]
    fn shl(self, rhs: i64) -> Self::Output {
        BitBoard((self.0).wrapping_shl(rhs as u32))
    }
}

impl Mul for BitBoard {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        BitBoard(self.0.wrapping_mul(rhs.0))
    }
}

impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        BitBoard(value)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const WIDTH: u8 = 8;

        let mut s: String = "".to_owned();
        let mut rank = 8;
        // could also traverse the ranks, files and determine bit from that
        for bit in 0u8..(u64::BITS as u8) {
            if bit % 8 == 0 {
                s.push_str(&format!("{rank} "));
                rank -= 1;
            }
            let pos = (0b111 - (bit >> 3)) << 3 | (bit & 0b111);
            s.push_str(if self.0 & (1 << pos) > 0 { "x " } else { ". " });
            if bit % WIDTH == WIDTH - 1 {
                s.push('\n');
            }
        }
        let files: String = ('a'..='h').fold(" ".to_owned(), |acc, e| format!("{acc} {e}"));
        s.push_str(&files);
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popcnt_empty_bitboard() {
        let empty_board = BitBoard::default();
        assert_eq!(empty_board.population_count(), 0)
    }

    #[test]
    fn popcnt_full_bitboard() {
        let full_bitboard = !BitBoard::default();
        assert_eq!(full_bitboard.population_count(), 64)
    }
}
