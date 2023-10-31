/// Convert a letter to its 0..26 index
///
/// Returns the index, and whether the letter was capitalised
pub fn letter_to_index(c: char) -> (usize, bool) {
    match c {
        'a'..='z' => ((c as usize) - ('a' as usize), false),

        'A'..='Z' => ((c as usize) - ('A' as usize), true),

        _ => panic!("Character {c:?} is not a valid letter"),
    }
}

pub fn index_to_letter(c: usize, capital: bool) -> char {
    char::from_u32(if capital {
        (c + 'A' as usize) as u32
    } else {
        (c + 'a' as usize) as u32
    })
    .unwrap()
}
