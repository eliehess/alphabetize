use std::fmt;

pub struct Card<'a> {
    name: &'a str,
    quantity: i32
}

impl Card<'_> {
    pub fn new<'a>(name: &'a str, quantity: i32) -> Card {
        Card {name, quantity}
    }

    pub fn get_name(&self) -> &'_ str {
        self.name
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.quantity, self.name)
    }
}

impl fmt::Display for Card<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn sort(mut list: Vec<Card>) -> Vec<Card> {
    list.sort_by(|a, b| a.name.cmp(b.name));
    list
}

pub fn join(list: &Vec<Card>) -> String {
    let mut str_vec = Vec::new();
    for card in list {
        str_vec.push(card.to_string())
    }
    str_vec.join("\n")
}
