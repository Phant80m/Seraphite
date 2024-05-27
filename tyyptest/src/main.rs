use std::io;
use std::io::Write;
use termion::color;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    println!("Type the sentence below:");
    let target_sentence = "the quick brown fox jumps over the lazy dog";

    let mut typed_sentence = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    print_sentence(&typed_sentence, target_sentence);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => {
                if c.is_alphabetic() || c == ' ' {
                    typed_sentence.push(c);
                    print_sentence(&typed_sentence, target_sentence);
                } else if c == '\n' {
                    if typed_sentence == target_sentence {
                        println!("\nYou typed the sentence correctly!");
                    } else {
                        println!("\nYou made mistakes, try again!");
                    }
                    break;
                }
            }
            Key::Backspace => {
                if !typed_sentence.is_empty() {
                    typed_sentence.pop();
                    print_sentence(&typed_sentence, target_sentence);
                }
            }
            Key::Ctrl('c') => break,
            _ => {}
        }
    }
}

fn print_sentence(typed: &str, target: &str) {
    let term_width = termion::terminal_size().unwrap().0 as usize;

    let target_words: Vec<&str> = target.split_whitespace().collect();
    let typed_words: Vec<&str> = typed.split_whitespace().collect();

    let mut typed_len = 0;
    let mut target_len = 0;

    for (typed_word, target_word) in typed_words.iter().zip(target_words.iter()) {
        typed_len += typed_word.len() + 1; // +1 for space
        target_len += target_word.len() + 1; // +1 for space
    }

    let start_column = (term_width - target_len) / 2;

    print!("{}", termion::cursor::Goto(start_column as u16, 1));

    for (typed_char, target_char) in typed.chars().zip(target.chars()) {
        if typed_char == target_char {
            print!("{}", color::Fg(color::Green));
            print!("{}", typed_char);
        } else {
            print!("{}", color::Fg(color::Red));
            print!("{}", target_char);
        }
    }

    if typed_len < target_len {
        let remaining_chars = &target[typed_len..];
        print!(
            "{}{}",
            termion::clear::UntilNewline,
            color::Fg(color::White)
        );
        print!("{}", remaining_chars);
    }

    print!("{}", color::Fg(color::Reset));

    // Calculate the position of the beam cursor
    let beam_cursor_pos = typed_len + start_column + 1; // Adjust position by 1 to account for one-based indexing
    print!("{}", Goto(beam_cursor_pos as u16, 1));

    io::stdout().flush().unwrap();
}
