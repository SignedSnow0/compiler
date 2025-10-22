pub trait Peek {
    type PeekValue;

    fn peek(&self) -> Option<Self::PeekValue>;
    fn trim_good(&mut self);
}

impl Peek for String {
    type PeekValue = char;

    fn peek(&self) -> Option<Self::PeekValue> {
        self.chars().next()
    }

    fn trim_good(&mut self) {
        while self.peek().is_some_and(|c| c == ' ') {
            self.remove(0);
        }
    }
}
