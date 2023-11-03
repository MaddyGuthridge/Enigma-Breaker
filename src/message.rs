use std::ops::{Deref, DerefMut};

use crate::Letter;

/// Represents a character from a message
///
/// The characters are encoded to be a `Letter` or a `char`, as this reduces
/// the amount of processing time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageChar {
    /// Character is a letter, bool represents whether it is capitalised
    /// (`true`) or not (`false`).
    Letter(Letter, bool),

    /// Character is not a letter, and is therefore not encoded
    Other(char),
}

impl From<char> for MessageChar {
    fn from(c: char) -> Self {
        if let Some((letter, capital)) = Letter::from_char(c) {
            MessageChar::Letter(letter, capital)
        } else {
            MessageChar::Other(c)
        }
    }
}

impl From<MessageChar> for char {
    fn from(val: MessageChar) -> Self {
        match val {
            MessageChar::Letter(l, capital) => l.to_char(capital),
            MessageChar::Other(c) => c,
        }
    }
}

/// Represents a parsed message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message(Vec<MessageChar>);

impl Deref for Message {
    type Target = Vec<MessageChar>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Message(value.chars().map(|c| c.into()).collect())
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        self.iter().map(|c| Into::<char>::into(c.clone())).collect()
    }
}
