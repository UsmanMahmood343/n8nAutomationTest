#[cfg(test)]
mod calculator;

use calculator::add;
use calculator::subtract;
use calculator::multiply;
use calculator::divide;

#[test]
fn test_add() {
    assert_eq!(add(2.0, 3.0), 5.0);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5.0, 2.0), 3.0);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(2.0, 3.0), 6.0);
}

#[test]
fn test_divide() {
    assert_eq!(divide(6.0, 3.0), 2.0);
    assert!(std::panic::catch_unwind(|| divide(1.0, 0.0)).is_err());
}