pub mod card {
    use std::fmt;
    use std::cmp;

    #[derive(Eq, Debug)]
    pub struct Card<'a> {
        name: &'a str,
        quantity: i32
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

        pub fn compare_to(&self, other: &Card) -> cmp::Ordering {
            return match self.name.cmp(other.name) {
                cmp::Ordering::Greater => cmp::Ordering::Greater,
                cmp::Ordering::Less => cmp::Ordering::Less,
                cmp::Ordering::Equal => self.quantity.cmp(&other.quantity)
            }
        }
    }

    impl fmt::Display for Card<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl PartialEq for Card<'_> {
        #[inline]
        fn eq(&self, other: &Card) -> bool {
            self.name.eq(other.name) && self.quantity == other.quantity
        }
    }

    impl PartialOrd for Card<'_> {
        fn partial_cmp(&self, other: &Card) -> Option<cmp::Ordering> {
            return match self.name.partial_cmp(other.name) {
                Some(ord) => match ord {
                    cmp::Ordering::Equal => self.quantity.partial_cmp(&other.quantity),
                    _ => Some(ord)
                }
                None => None
            }
        }
    }

    impl Ord for Card<'_> {
        fn cmp(&self, other: &Card) -> cmp::Ordering {
            return match self.name.cmp(other.name) {
                cmp::Ordering::Equal => self.quantity.cmp(&other.quantity),
                cmp::Ordering::Greater => cmp::Ordering::Greater,
                cmp::Ordering::Less => cmp::Ordering::Less
            }
        }
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
                continue;
            }
            let quantity: i32 = match line[0..index].parse::<i32>() {
                Ok(val) => val,
                Err(e) => {
                    errors.push(format!("{}", e));
                    continue;
                }
            };
            let name: &str = &line[index+1..line.len()];
            cards.push(Card::new(quantity, name));
        }

        return (cards, errors);
    }
}

#[cfg(test)]
mod tests {
    use super::card::*;

    #[test]
    fn empty_input() {
        let (cards, errors) = parse("");
        assert_eq!(cards.len(), 0);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn correct_parse() {
        let (cards, errors) = parse("4 Thoughtseize\n4 Brainstorm");
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0], Card::new(4, "Thoughtseize"));
        assert_eq!(cards[1], Card::new(4, "Brainstorm"));
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn correct_parse_with_trailing_newline() {
        let (cards, errors) = parse("4 Thoughtseize\n4 Brainstorm\n");
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0], Card::new(4, "Thoughtseize"));
        assert_eq!(cards[1], Card::new(4, "Brainstorm"));
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_nums() {
        let (cards, errors) = parse("Thoughtseize\n4 Brainstorm");
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], Card::new(4, "Brainstorm"));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], String::from("No spaces found in line"));
    }

    #[test]
    fn no_spaces() {
        let (cards, errors) = parse("4Thoughtseize\n4 Brainstorm");
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], Card::new(4, "Brainstorm"));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], String::from("No spaces found in line"));
    }
    
    #[test]
    fn not_numbers() {
        let (cards, errors) = parse("a Thoughtseize\n4 Brainstorm");
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].get_name(), "Brainstorm");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], String::from("invalid digit found in string"));
    }

    #[test]
    fn sorts_cards() {
        let mut cards = vec!(Card::new(4, "Thoughtseize"), Card::new(4, "Brainstorm"));
        cards.sort();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0], Card::new(4, "Brainstorm"));
        assert_eq!(cards[1], Card::new(4, "Thoughtseize"));
    }

    #[test]
    fn sorts_cards_mixed_case() {
        let mut cards = vec!(Card::new(4, "thoughtseize"), Card::new(4, "Brainstorm"));
        cards.sort();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0], Card::new(4, "Brainstorm"));
        assert_eq!(cards[1], Card::new(4, "thoughtseize"));
    }
}
