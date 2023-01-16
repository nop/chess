//! Representation for board locations.

use std::str::FromStr;

/// A `Square` represents a pair of [`Rank`] and [`File`] that describes a location
/// on the [`Board`].
///
/// [`Rank`]: Rank
/// [`File`]: File
/// [`Board`]: super::Board
#[derive(Debug)]
pub struct Square {
    /// The `File` this `Square` resides on.
    pub file: File,
    /// The `Rank` this `Square` resides on.
    pub rank: Rank,
}

impl From<Square> for u8 {
    /// Map a `Square` to a u8 index for use in `BitBoard`.
    fn from(value: Square) -> Self {
        let rank = value.rank as u8;
        let file: u8 = value.file.into();

        const WIDTH: u8 = 8;

        ((rank - 1) * WIDTH) + file - 1
    }
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(format!("Length is not 2; len: {}, &str: {}", s.len(), s));
        }
        if !s.is_ascii() {
            return Err("Non-ASCII input".to_owned());
        }

        let mut cs = s.chars();
        let Some(file) = cs.next() else {
            return Err("Empty first character.".to_owned())
        };
        let Some(rank) = cs.next() else {
            return Err("Empty second character.".to_owned())
        };

        Ok(Square {
            rank: rank.try_into().expect("unable to parse rank"),
            file: file.try_into().expect("unable to parse file"),
        })
    }
}

/// A row of the chessboard.
///
/// In algebraic notation, [rank]s are numbered 1–8 starting from White's side of the board.
/// For example: White's king and other pieces start on their first (or "back" or "home") rank,
/// whereas Black calls the same rank the "eighth" (or last) rank.
///
/// [Rank]: https://en.wikipedia.org/wiki/Glossary_of_chess#rank
#[derive(Debug)]
pub enum Rank {
    #[allow(missing_docs)]
    One = 1,
    #[allow(missing_docs)]
    Two,
    #[allow(missing_docs)]
    Three,
    #[allow(missing_docs)]
    Four,
    #[allow(missing_docs)]
    Five,
    #[allow(missing_docs)]
    Six,
    #[allow(missing_docs)]
    Seven,
    #[allow(missing_docs)]
    Eight,
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::Rank::*;

        match value {
            1 => Ok(One),
            2 => Ok(Two),
            3 => Ok(Three),
            4 => Ok(Four),
            5 => Ok(Five),
            6 => Ok(Six),
            7 => Ok(Seven),
            8 => Ok(Eight),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if ('1'..='8').contains(&value) {
            return Ok((value.to_digit(10).unwrap() as u8).try_into().unwrap());
        }
        Err(String::new())
    }
}

/// A column of the chessboard.
///
/// Each [file] is named using its position in algebraic notation, a–h.
///
/// [file]: https://en.wikipedia.org/wiki/Glossary_of_chess#file
#[derive(Debug)]
pub enum File {
    #[allow(missing_docs)]
    A = 1,
    #[allow(missing_docs)]
    B,
    #[allow(missing_docs)]
    C,
    #[allow(missing_docs)]
    D,
    #[allow(missing_docs)]
    E,
    #[allow(missing_docs)]
    F,
    #[allow(missing_docs)]
    G,
    #[allow(missing_docs)]
    H,
}

impl TryFrom<u8> for File {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::File::*;

        match value {
            1 => Ok(A),
            2 => Ok(B),
            3 => Ok(C),
            4 => Ok(D),
            5 => Ok(E),
            6 => Ok(F),
            7 => Ok(G),
            8 => Ok(H),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for File {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let lowercase = value.to_lowercase().next().expect("Multi-byte ToLowercase");
        Self::try_from(lowercase as u8 - b'a' + 1)
    }
}

impl From<File> for u8 {
    fn from(value: File) -> Self {
        use self::File::*;

        match value {
            A => 1,
            B => 2,
            C => 3,
            D => 4,
            E => 5,
            F => 6,
            G => 7,
            H => 8,
        }
    }
}
