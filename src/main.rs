mod trie;
mod reader;

use trie::Trie;

fn main() {
    let mut trie: Trie = Trie::new(); 
    let words: Vec<String> = vec!["abc", "abcde", "abcdefg", "abc123"].iter().map(|x| x.to_string()).collect();

    reader::read_from_file("./src/data/words.txt", &mut trie);
    
    println!("TOP K is {:?}", trie.get_top_k("p".to_string(), 10));
    println!("LEN IS {:?}", trie.len());
}
