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
pub struct Trie {
    root: Node,
    count: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node::new(),
            count: 0,
        }
    } 

    pub fn add(&mut self, w: String) {
        let mut cur: &mut Node = &mut self.root;
        for c in w.chars() {
            cur = cur.children.entry(c).or_insert(Node::new()); 
        }
        if !cur.end {
            cur.end = true; 
            self.count += 1;
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, w: &String) -> bool {
        let mut cur: &Node = &self.root;
        for c in w.chars() {
            match cur.children.get(&c) {
                Some(node) => cur = node,
                None => return false,
            }    
        }
        cur.end
    }

    pub fn get_top_k(&self, word: &String, mut k: usize) -> Option<Vec<String>> {
        if word.is_empty() {
            return None;
        }

        let mut cur: &Node = &self.root; 
        for c in word.chars() {
            match cur.children.get(&c) {
                Some(node) => cur = node,
                None => return None,
            }
        }

        let mut ans: Vec<String> = Vec::with_capacity(k);

        self.dfs(cur, word, &mut ans, &mut k);
        Some(ans)
    }

    fn dfs(&self, node: &Node, word: &String, vec: &mut Vec<String>, k: &mut usize) {
        if *k == 0 {
            return;
        }

        if (*node).end {
            (*vec).push(word.clone()); 
            *k -= 1;
        }
        
        for (key, child_node) in &(*node).children {
            // TODO: not a fan of this clone here. See if we can get rid of it?
            let mut w: String = word.clone();
            w.push(*key);
            self.dfs(&child_node, &w, vec, k);
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }
}
