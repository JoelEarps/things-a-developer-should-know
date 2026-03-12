//! Swap two integers without using a third variable.
//!
//! Example: x = 10, y = 78 → x = 78, y = 10

/// Swap using addition and subtraction.
/// Note: can overflow for large values; use swap_xor if that's a concern.
pub fn swap_arithmetic(x: &mut i32, y: &mut i32) {
    *x = *x + *y;
    *y = *x - *y;
    *x = *x - *y;
}

/// Swap using XOR. No overflow, works for any integer type.
pub fn swap_xor(x: &mut i32, y: &mut i32) {
    *x = *x ^ *y;
    *y = *x ^ *y;
    *x = *x ^ *y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_arithmetic_example() {
        let mut x = 10;
        let mut y = 78;
        swap_arithmetic(&mut x, &mut y);
        assert_eq!(x, 78);
        assert_eq!(y, 10);
    }

    #[test]
    fn swap_arithmetic_same_value() {
        let mut x = 42;
        let mut y = 42;
        swap_arithmetic(&mut x, &mut y);
        assert_eq!(x, 42);
        assert_eq!(y, 42);
    }

    #[test]
    fn swap_arithmetic_negatives() {
        let mut x = -5;
        let mut y = 20;
        swap_arithmetic(&mut x, &mut y);
        assert_eq!(x, 20);
        assert_eq!(y, -5);
    }

    #[test]
    fn swap_xor_example() {
        let mut x = 10;
        let mut y = 78;
        swap_xor(&mut x, &mut y);
        assert_eq!(x, 78);
        assert_eq!(y, 10);
    }

    #[test]
    fn swap_xor_same_value() {
        let mut x = 42;
        let mut y = 42;
        swap_xor(&mut x, &mut y);
        assert_eq!(x, 42);
        assert_eq!(y, 42);
    }

    #[test]
    fn swap_xor_negatives() {
        let mut x = -5;
        let mut y = 20;
        swap_xor(&mut x, &mut y);
        assert_eq!(x, 20);
        assert_eq!(y, -5);
    }

    #[test]
    fn swap_xor_zero() {
        let mut x = 0;
        let mut y = 100;
        swap_xor(&mut x, &mut y);
        assert_eq!(x, 100);
        assert_eq!(y, 0);
    }
}
