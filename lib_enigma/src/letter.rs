use num::FromPrimitive;
use num_derive::FromPrimitive as DeriveFromPrimitive;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};
use strum::{EnumCount, EnumIter};

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, EnumCount, EnumIter, DeriveFromPrimitive,
)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

unsafe impl Send for Letter {}

impl Letter {
    /// Create from a usize, which must be in range 0..26
    pub fn from_usize(n: usize) -> Option<Letter> {
        FromPrimitive::from_usize(n)
    }

    /// Convert a char to its 0..26 index if it's a letter
    ///
    /// Returns the index, and whether the letter was capitalised
    pub fn from_char(c: char) -> Option<(Letter, bool)> {
        match c {
            'a'..='z' => Some((
                Self::from_usize((c as usize) - ('a' as usize)).unwrap(),
                false,
            )),

            'A'..='Z' => Some((
                Self::from_usize((c as usize) - ('A' as usize)).unwrap(),
                true,
            )),

            _ => None,
        }
    }

    /// Convert a letter back to its character representation
    pub fn to_char(self, capital: bool) -> char {
        char::from_u32(if capital {
            (self as usize + 'A' as usize) as u32
        } else {
            (self as usize + 'a' as usize) as u32
        })
        .unwrap()
    }
}

impl Add<usize> for Letter {
    type Output = Letter;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from_usize((self as usize + rhs) % Self::COUNT).unwrap()
    }
}

impl AddAssign<usize> for Letter {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl Sub<usize> for Letter {
    type Output = Letter;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::from_usize((self as usize + Self::COUNT - rhs) % Self::COUNT).unwrap()
    }
}

impl SubAssign<usize> for Letter {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs;
    }
}

impl Add<Letter> for Letter {
    type Output = Letter;

    fn add(self, rhs: Letter) -> Self::Output {
        Self::from_usize((self as usize + rhs as usize) % Self::COUNT).unwrap()
    }
}

impl Sub<Letter> for Letter {
    type Output = Letter;

    fn sub(self, rhs: Letter) -> Self::Output {
        Self::from_usize((self as usize + Self::COUNT - rhs as usize) % Self::COUNT).unwrap()
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char(true))
    }
}

#[cfg(test)]
mod test {
    use crate::letter::Letter;

    #[test]
    fn letter_to_number() {
        assert_eq!(Letter::A as usize, 0);
    }

    #[test]
    fn add_letters() {
        assert_eq!(Letter::A + 1, Letter::B);
    }

    #[test]
    fn add_letters_wrap() {
        assert_eq!(Letter::Z + 1, Letter::A);
    }

    #[test]
    fn sub_letters() {
        assert_eq!(Letter::B - 1, Letter::A);
    }

    #[test]
    fn sub_letters_wrap() {
        assert_eq!(Letter::A - 1, Letter::Z);
    }
}
