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

    pub fn parse(text: &str) -> (Vec<Card>, Vec<String>) {
        let mut cards: Vec<Card> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        let lines: Vec<&str> = text.clone().lines().collect();

        for line in lines {
            let index: usize;
            if let Some(a) = line.find(" ") {
                index = a;
            } else {
                errors.push(String::from("No spaces found in line"));
                break;
            }
            let quantity: i32 = match line[0..index].parse::<i32>() {
                Ok(val) => val,
                Err(e) => {
                    errors.push(format!("Unable to parse quantity - {}", e));
                    break;
                }
            };
            let name: &str = &line[index+1..line.len()];
            cards.push(Card::new(quantity, name));
        }

        return (cards, errors);
    }
}