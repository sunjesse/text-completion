use std::collections::HashMap;

#[derive(Default, Debug)]
struct Node {
    end: bool,
    children: HashMap<char, Node>,
}

impl Node {
    fn new() -> Self {
        Node::default()
    }
}

#[derive(Debug)]
struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Node::new(),
        }
    } 

    fn add(&mut self, w: String) {
        let mut cur: &mut Node = &mut self.root;
        for c in w.chars() {
            cur = cur.children.entry(c).or_insert(Node::new()); 
        }
        cur.end = true; 
    }

    fn contains(&self, w: &String) -> bool {
        let mut cur: &Node = &self.root;
        for c in w.chars() {
            match cur.children.get(&c) {
                Some(node) => cur = node,
                None => return false,
            }    
        }
        cur.end
    }

    fn get_top_k(&self, w: String, mut k: usize) -> Option<Vec<String>> {
        let mut cur: &Node = &self.root; 
        for c in w.chars() {
            match cur.children.get(&c) {
                Some(node) => cur = node,
                None => return None,
            }
        }

        let mut ans: Vec<String> = Vec::with_capacity(k);

        self.dfs(cur, w, &mut ans, &mut k);
        Some(ans)
    }

    fn dfs(&self, node: &Node, word: String, vec: &mut Vec<String>, k: &mut usize) {
        if *k == 0 {
            return;
        }

        if (*node).end {
            (*vec).push(word.clone()); 
            *k -= 1;
        }
        
        for (key, child_node) in &(*node).children {
            let mut w: String = word.clone();
            w.push(*key);
            self.dfs(&child_node, w, vec, k);
        }
        
    }
}


fn main() {
    let mut trie: Trie = Trie::new(); 
    let words: Vec<String> = vec!["abc", "abcde", "abcdefg", "abc123"].iter().map(|x| x.to_string()).collect();
    for w in words {
        trie.add(w);
    }
    
    println!("TOP K is {:?}", trie.get_top_k("abc".to_string(), 4));
}
