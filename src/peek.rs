pub trait Peek {
    type PeekValue;

    fn peek(&self) -> Option<Self::PeekValue>;
}

impl Peek for String {
    type PeekValue = char;

    fn peek(&self) -> Option<Self::PeekValue> {
        self.chars().next()
    }
}
