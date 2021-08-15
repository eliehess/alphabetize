pub mod card {
    use std::fmt;

    pub struct Card<'a> {
        quantity: i32,
        name: &'a str
    }

    impl Card<'_> {
        pub fn new<'a>(quantity: i32, name: &'a str) -> Card {
            Card {quantity, name}
        }

        pub fn get_quantity(&self) -> i32 {
            self.quantity
        }

        pub fn get_name(&self) -> &'_ str {
            self.name
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

    pub fn sort(list: &mut Vec<Card>) {
        list.sort_by(|a, b| a.name.cmp(b.name));
    }

    pub fn join(list: &Vec<Card>) -> String {
        let mut str_vec = Vec::new();
        for card in list {
            str_vec.push(card.to_string())
        }
        return str_vec.join("\n");
    }
}