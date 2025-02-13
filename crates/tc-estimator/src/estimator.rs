use std::collections::BTreeMap;
use std::cmp::Reverse;
use rand::Rng;

#[derive(Debug)]
struct Link {
    next: BTreeMap<String, u32>,
    denom: u32,
}

impl Link {
    fn new() -> Self {
        Link {
            next: BTreeMap::new(),
            denom: 0,
        }
    }

    fn add(&mut self, w: String) {
        *self.next.entry(w.to_lowercase()).or_insert(0) += 1;
        self.denom += 1;
    }

    fn sample(&self) -> String {
        let t: u32 = rand::rng().random_range(0..self.denom) as u32;
        let mut pf: u32 = 0;
        for (k, v) in self.next.iter() {
            pf += v;
            if pf > t {
                return k.clone();
            } 
        }
        unreachable!();
    }
    
    fn range(&self) -> Vec<String> {
        let mut kv: Vec<_> = self.next.iter().collect();
        kv.sort_by_key(|&(_, v)| Reverse(v));
        kv.into_iter().take(10).map(|(k, _)| k.clone()).collect()
    }
}


#[derive(Debug)]
pub struct Estimator {
    words: BTreeMap<String, Link>, 
}

impl Estimator {
    pub fn new() -> Self {
        Estimator {
            words: BTreeMap::new(),
        }
    }

    fn add_word(&mut self, w: String) {
        if !self.words.contains_key(&w) {
            let link = Link::new();
            self.words.insert(w, link); 
        }
    }

    pub fn add_link(&mut self, w1: &String, w2: &String) {
        let w1 = w1.to_lowercase();
        let w2 = w2.to_lowercase();
        self.add_word(w1.clone());
        if let Some(link) = self.words.get_mut(&w1) {
            link.add(w2.clone()); 
        }
    }

    pub fn predict(&self, w: String) -> Option<String> {
        let w = w.to_lowercase();
        if let Some(link) = self.words.get(&w.to_lowercase()) {
            return Some(link.sample());
        }
        None
    }

    pub fn range(&self, w: String) -> Option<Vec<String>> {
        let w = w.to_lowercase();
        if let Some(link) = self.words.get(&w.to_lowercase()) {
            return Some(link.range());     
        }
        None
    }
}
