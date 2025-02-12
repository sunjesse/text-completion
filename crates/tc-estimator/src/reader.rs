use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::Estimator;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_from_file(path: &str, est: &mut Estimator) {
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            if !line.is_empty() {
                let split_line: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
                println!("{:?}", split_line);
                for i in 0..(split_line.len()-1) {
                    println!("adding {:?} {:?}", split_line[i], split_line[i+1]);
                    est.add_link(&split_line[i], &split_line[i+1]);
                }
            }
        }
    }
}
