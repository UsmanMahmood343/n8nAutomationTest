pub fn sqrt(a: f64) -> f64 {
    if a < 0.0 {
        panic!("Cannot take square root of a negative number");
    }
    a.sqrt()
}