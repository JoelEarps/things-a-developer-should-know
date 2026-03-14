# Pulsar take-home: Swap two integers without a third variable

**Problem:** Given two integers, write a function to swap them without using any third variable.

**Example:**
- Input: X = 10, Y = 78  
- Output: X = 78, Y = 10

---

## Solution: XOR swap

We use the **XOR swap** algorithm. It uses the fact that XOR is reversible: `(a ^ b) ^ b = a` and `(a ^ b) ^ a = b`.

### How it works

Let `x` and `y` be the two integers. We do three XOR assignments in place:

1. **`x = x ^ y`**  
   `x` now holds `(original_x ^ original_y)`. We haven’t lost any information: the combined “difference” is in `x`.

2. **`y = x ^ y`**  
   `x` is `(original_x ^ original_y)` and `y` is still `original_y`.  
   So `x ^ y = (original_x ^ original_y) ^ original_y = original_x` (XOR with same value twice cancels).  
   So `y` becomes `original_x`.

3. **`x = x ^ y`**  
   `x` is still `(original_x ^ original_y)` and `y` is now `original_x`.  
   So `x ^ y = (original_x ^ original_y) ^ original_x = original_y`.  
   So `x` becomes `original_y`.

After these three steps: `x = original_y` and `y = original_x`. Swapped, with no extra variable.

### Why XOR?

- **Reversible:** `a ^ b ^ b = a`, so we can recover both values from the “combined” value and one original.
- **No overflow:** Unlike an add/subtract trick (`x = x + y; y = x - y; x = x - y`), XOR doesn’t overflow.
- **Works for any integer type:** Same idea works for `i32`, `u64`, etc.

### Code

See `src/main.rs` (or the snippet below). Run with `cargo run`.

```rust
fn swap_without_temp(x: &mut i32, y: &mut i32) {
    *x ^= *y;
    *y ^= *x;
    *x ^= *y;
}
```

### Note

In practice, modern compilers optimise a normal three-variable swap to the same or better code, and the XOR version can be harder for people to read. So this is mainly a “do it without a temp variable” puzzle; the XOR trick is the standard way to satisfy that constraint.
