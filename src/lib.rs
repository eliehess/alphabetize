use std::fmt;

pub struct Card<'a> {
    name: &'a str,
    quantity: i32
}

impl Card<'_> {
    pub fn new<'a>(name: &'a str, quantity: i32) -> Card {
        Card {name, quantity}
    }
}

impl fmt::Display for Card<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, self.name)
    }
}
