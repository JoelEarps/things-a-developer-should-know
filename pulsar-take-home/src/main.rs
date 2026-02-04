//! Pulsar take-home: Swap two integers without a third variable.
//!
//! Uses the XOR swap algorithm. See README.md for a full explanation.

/// Swaps the values in `x` and `y` in place without using a third variable.
///
/// **How it works:**
/// 1. `x ^= y`  →  x now holds (original_x ^ original_y)
/// 2. `y ^= x`  →  y = (original_x ^ original_y) ^ original_y = original_x
/// 3. `x ^= y`  →  x = (original_x ^ original_y) ^ original_x = original_y
/// Result: x = original_y, y = original_x.
fn swap_without_temp(x: &mut i32, y: &mut i32) {
    *x ^= *y;
    *y ^= *x;
    *x ^= *y;
}

fn main() {
    let mut x = 10_i32;
    let mut y = 78_i32;

    println!("Before: X = {}, Y = {}", x, y);
    swap_without_temp(&mut x, &mut y);
    println!("After:  X = {}, Y = {}", x, y);

    // Example from problem statement
    assert_eq!(x, 78);
    assert_eq!(y, 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_example() {
        let mut x = 10;
        let mut y = 78;
        swap_without_temp(&mut x, &mut y);
        assert_eq!(x, 78);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_swap_symmetric() {
        let mut a = 42;
        let mut b = -17;
        swap_without_temp(&mut a, &mut b);
        assert_eq!(a, -17);
        assert_eq!(b, 42);
    }

    #[test]
    fn test_swap_same_value() {
        let mut a = 7;
        let mut b = 7;
        swap_without_temp(&mut a, &mut b);
        assert_eq!(a, 7);
        assert_eq!(b, 7);
    }
}
