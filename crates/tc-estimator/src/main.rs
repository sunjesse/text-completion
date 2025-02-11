use std::collections::BTreeMap;
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
        *self.next.entry(w).or_insert(0) += 1;
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
}


#[derive(Debug)]
struct Estimator {
    words: BTreeMap<String, Link>, 
}

impl Estimator {
    fn new() -> Self {
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

    fn add_link(&mut self, w1: &String, w2: &String) {
        self.add_word(w1.clone());
        if let Some(link) = self.words.get_mut(w1) {
            link.add(w2.clone()); 
        }
    }

    fn predict(&self, w: String) -> Option<String> {
        if let Some(link) = self.words.get(&w) {
            return Some(link.sample());
        }
        None
    }
}

fn main() {
    let sentence: Vec<String> = ["hi", "my", "name", "my", "age"].iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut est = Estimator::new();
    for i in 0..(sentence.len()-1) {
        est.add_link(&sentence[i], &sentence[i+1]); 
    }
    println!("{:?}", est);
    for _ in 0..10 {
        println!("PRED: {:?}", est.predict("my".to_string()));
    }
}
