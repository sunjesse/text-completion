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

    println!("Vocab length is: {:}.", trie.len());
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    let mut input: String = String::new();
    let mut input_words: Vec<String> = Vec::new();
    let mut render_string = String::new();

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                execute!(stdout, Clear(ClearType::All))?;
                match key_event.code {
                    KeyCode::Char(c) => {
                        if c == ' ' {
                            if !input.is_empty() {
                                input_words.push(input.clone());
                                if !render_string.is_empty() {
                                    render_string.push_str(" ");
                                }
                                render_string.push_str(&input);
                            }
                            input.clear();
                        } else {
                            input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                        } else if let Some(w) = input_words.pop() {
                            input = w;
                            render_string = input_words.join(" ");
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
                // print suggestions
                if let Some(words) = trie.get_top_k(input.clone(), 10) {
                    println!("\r{:?}", words);
                }

                // print buffered words
                if input_words.is_empty() {
                    println!("\r{}", input);
                } else {
                    println!("\r{} {}", render_string, input);
                }
                stdout.flush()?;
            }
        }
    }
    disable_raw_mode()?;
    Ok(())

}
