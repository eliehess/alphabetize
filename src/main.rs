use alphabetize::card;
use clipboard_win::Clipboard;
use std::io;

fn main() {
    let raw_input = match get_clipboard() {
        Ok(text) => text,
        Err(e) =>  {
            handle_errors(&vec!(format!("{}", e)));
            return;
        }
    };

    let (mut cards, mut errors) = card::parse(&raw_input);

    if cards.len() < 1 {
        errors.push(String::from("No cards parsed"));
        handle_errors(&errors);
        return;
    }

    cards.sort();

    match set_clipboard(&card::join(&cards)) {
        Ok(()) => (),
        Err(e) => errors.push(format!("{}", e))
    };

    if errors.len() == 0 {
        println!("done");
    } else {
        handle_errors(&errors);
    }
}

fn handle_errors(errors: &Vec<String>) {
    for error in errors {
        println!("{}", error);
    }
}

fn get_clipboard() -> Result<String, io::Error> {
    match Clipboard::new() {
        Ok(clipboard) => clipboard.get_string(),
        Err(e) => Err(e)
    }
}

fn set_clipboard(text: &str) -> Result<(), io::Error> {
    match Clipboard::new() {
        Ok(clipboard) => 
            match clipboard.set_string(text) {
                Ok(()) => Ok(()),
                Err(e) => Err(e)
            },
        Err(e) => Err(e)
    }
}
