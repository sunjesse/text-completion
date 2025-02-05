mod trie;
mod reader;

use trie::Trie;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::execute;

fn main() -> std::io::Result<()> {
    let mut trie: Trie = Trie::new(); 
    reader::read_from_file("./src/data/words.txt", &mut trie);

    println!("LEN IS {:?}", trie.len());
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    let mut input: String = String::new();
    let mut input_words: Vec<String> = Vec::new();

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                execute!(stdout, Clear(ClearType::All))?;
                match key_event.code {
                    KeyCode::Char(c) => {
                        if c == ' ' {
                            if !input.is_empty() {
                                input_words.push(input.clone());
                            }
                            input.clear();
                        } else {
                            input.push(c);
                        }
                        println!("\r{} {}", input_words.join(" "), input);
                        stdout.flush()?;
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            stdout.flush()?;
                        } else if !input_words.is_empty() {
                            input = input_words.pop().expect("did not pop");
                        }
                        println!("\r{} {}", input_words.join(" "), input);
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
                if let Some(words) = trie.get_top_k(input.clone(), 10) {
                    println!("\r{:?}", words);
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())

}
