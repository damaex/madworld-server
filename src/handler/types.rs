use std::fmt;

pub enum MessageMainType {
    LOGIN = 0,
    TALK = 1,
    ACTION = 2,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Message {
    pub main: i32,
    pub sub: i32,
    pub data: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "(MainType: {}, SubType: {}, Data: {})", self.main, self.sub, self.data)
    }
}