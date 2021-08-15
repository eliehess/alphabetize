use alphabetizer_rust::Card;
use clipboard_win::Clipboard;

fn main() {
    let raw_input = Clipboard::new().unwrap().get_string().unwrap();
    let lines: Vec<&str> = raw_input.lines().collect();
    
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        let index: usize;
        if let Some(a) = line.find(" ") {
            index = a;
        } else {
            println!("ERROR: No spaces found in line");
            break;
        }
        let quantity: i32 = line[0..index].parse::<i32>().unwrap();
        let name: &str = &line[index+1..line.len()];
        cards.push(Card::new(name, quantity));
    }

    cards = alphabetizer_rust::sort(cards);

    Clipboard::new().unwrap().set_string(&alphabetizer_rust::join(&cards)).unwrap();
}
