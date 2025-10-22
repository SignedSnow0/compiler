use crate::peek::Peek;

pub trait Token {
    fn parse(string: String) -> Option<(Self, String)>
    where
        Self: Sized;
    fn starter_set() -> Vec<char>
    where
        Self: Sized;

    fn from_string(string: String) -> Option<(Self, String)>
    where
        Self: Sized,
    {
        if let Some(character) = string.peek() {
            if Self::starter_set().iter().any(|c| *c == character) {
                return Self::parse(string);
            } else {
                return None;
            }
        }

        None
    }
}
