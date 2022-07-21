use tcod::colors::*;

pub struct Messages {
    messages: Vec<(String, Color)>,
}

impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    // The <T: Into<String>> bit makes the add function generic.
    // Instead of accepting a parameter of a specified type,
    // it can work with anything that implements the Into trait for String,
    // i.e. anything that can be converted to String.
    // This lets us pass both &str (and therefore string literals)
    // and String (an output of the format! macro among other things).
    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color))
    }

    // As we’re keeping the inner messages field private,
    // we need to provide a way for our users to access the messages.
    // In Rust, this is typically done via iterators.
    // "This function returns some type implementing this trait"
    // and let the compiler figure it out.
    // DoubleEndedIterator is a Trait.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item=&(String, Color)> {
        self.messages.iter()
    }
}