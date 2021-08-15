use alphabetizer_rust::card;
use clipboard_win::Clipboard;

fn main() {
    let raw_input = Clipboard::new().unwrap().get_string().unwrap();
    let lines: Vec<&str> = raw_input.lines().collect();
    
    let mut cards: Vec<card::Card> = Vec::new();
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
        cards.push(card::Card::new(quantity, name));
    }

    card::sort(&mut cards);

    Clipboard::new().unwrap().set_string(&card::join(&cards)).unwrap();
}
