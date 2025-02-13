use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::Trie;
use tc_estimator::Estimator;

fn cleanse(s: &mut String) {
    while matches!(s.chars().last(), Some('\n' | '.' | ',' | '!')) {
        s.pop();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// The following is for loading to trie only.
#[allow(dead_code)]
pub fn read_from_file(path: &str, trie: &mut Trie) {
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            trie.add(line); 
        }
    }
}

pub fn read_to_trie_est(path: &str, trie: &mut Trie, est: &mut Estimator) {
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            if !line.is_empty() {
                let split_line: Vec<String> = line
                    .split_whitespace()
                    .map(|x| {
                        let mut w = x.to_string();
                        cleanse(&mut w);
                        w
                    })
                    .collect::<Vec<String>>();

                for i in 0..(split_line.len()-1) {
                    est.add_link(&split_line[i], &split_line[i+1]);
                    trie.add(split_line[i].clone());
                }
                trie.add(split_line[split_line.len()-1].clone());
            }
        }
    }
}
