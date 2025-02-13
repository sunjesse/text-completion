mod estimator;
mod reader;

use estimator::Estimator;

pub fn load_estimator(path: &str) -> Estimator {
    let mut est = Estimator::new();
    reader::read_from_file(path, &mut est);
    est
}
