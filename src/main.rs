use alphabetizer_rust::card;
use clipboard_win::Clipboard;
use std::io;

fn main() {
    let mut errors: Vec<String> = Vec::new();
    let raw_input = match get_clipboard() {
        Ok(text) => text,
        Err(e) =>  {
            handle_errors(vec!(format!("{}", e)));
            return;
        }
    };
    let lines: Vec<&str> = raw_input.lines().collect();
    
    let mut cards: Vec<card::Card> = Vec::new();
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
        cards.push(card::Card::new(quantity, name));
    }

    if cards.len() < 1 {
        errors.push(String::from("No cards parsed"));
        handle_errors(errors);
        return;
    }

    card::sort(&mut cards);

    match set_clipboard(&card::join(&cards)) {
        Ok(()) => (),
        Err(e) => errors.push(format!("{}", e))
    };

    handle_errors(errors);
}

fn handle_errors(errors: Vec<String>) {
    for error in errors {
        println!("{}", error);
    }
} 

fn get_clipboard() -> Result<String, io::Error> {
    match Clipboard::new() {
        Ok(clipboard) => return clipboard.get_string(),
        Err(e) => return Err(e)
    };
}

fn set_clipboard(text: &str) -> Result<(), io::Error> {
    match Clipboard::new() {
        Ok(clipboard) => {
            match clipboard.set_string(text) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e)
            }
        },
        Err(e) => return Err(e)
    };
}
