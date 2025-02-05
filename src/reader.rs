use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::Trie;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_from_file(path: &str, trie: &mut Trie) {
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            trie.add(line); 
        }
    }
}
