pub trait StringUtils {
    type PeekValue;

    fn peek(&self) -> Option<Self::PeekValue>;
    fn trim_mut(self) -> Self;
}

impl StringUtils for String {
    type PeekValue = char;

    fn peek(&self) -> Option<Self::PeekValue> {
        self.chars().next()
    }

    fn trim_mut(mut self) -> Self {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.remove(0);
        }
        self
    }
}
