mod estimator;
mod reader;

use estimator::Estimator;
use std::io;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    let mut est = Estimator::new();
    reader::read_from_file("./src/data/rust-lang.txt", &mut est);

    loop {
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to get cin");
        trim_newline(&mut x);
        if x == "kekw" {
            break;
        }
        println!("{:?}", est.range(x));
    }
}
