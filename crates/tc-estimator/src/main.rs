mod estimator;
mod reader;

use estimator::Estimator;

fn main() {
    let mut est = Estimator::new();
    reader::read_from_file("./src/data/rust-lang.txt", &mut est);

    println!("Esimator: {:?}", est);
    for _ in 0..100 {
        println!("PRED: {:?}", est.predict("it".to_string()));
    }
    println!("{:?}", est.range("it".to_string()));
}
