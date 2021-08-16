use alphabetizer_rust::card;
use clipboard_win::Clipboard;
use std::io;

fn main() {
    let raw_input = match get_clipboard() {
        Ok(text) => text,
        Err(e) =>  {
            handle_errors(vec!(format!("{}", e)));
            return;
        }
    };
    
    let (mut cards, mut errors) = card::parse(&raw_input);

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
    return match Clipboard::new() {
        Ok(clipboard) => clipboard.get_string(),
        Err(e) => Err(e)
    };
}

fn set_clipboard(text: &str) -> Result<(), io::Error> {
    return match Clipboard::new() {
        Ok(clipboard) => 
            match clipboard.set_string(text) {
                Ok(()) => Ok(()),
                Err(e) => Err(e)
            },
        Err(e) => Err(e)
    };
}
