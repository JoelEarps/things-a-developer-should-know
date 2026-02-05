//! # Question:
//! Design a Data Structure SpecialStack that supports all the stack operations
//! like push(), pop(), isEmpty() and an additional operation getMin() which
//! should return the minimum element from the SpecialStack. All these operations
//! of SpecialStack must be O(1).
//!
//! Consider the following SpecialStack:
//! 16  <-- TOP
//! 15
//! 29
//! 19
//! 18
//! When getMin() is called it should return 15, which is the minimum element
//! in the current stack.
//!
//! If we do pop two times on stack, the stack becomes:
//! 29  <-- TOP
//! 19
//! 18
//! When getMin() is called, it should return 18, which is the minimum in the
//! current stack.
//!
//! Time complexity:
//! - Aim for O(1) time for push, pop, is_empty, get_min.

use std::collections::VecDeque;


/// Thought process, there are two main problems:
/// 1. We need a structure that can store the order in which values are pushed or popped onto the stack
/// 2. We need a structure that can store the minimum values in order i.e store in ascending order
/// However there is no such structure that allows us to do both of these things in O(1) time.
/// Therefore we need to use a combination of a stack and a min stack.
/// In order to do this I would use two VecDeques to store the values and the minimum values respectively.
/// Storing values - as we only need 4 functions we don't need to worry about inserting in the middle, therefore we can implement pop and push as a wrapper
/// However we do need to worry about the minimum value, therefore we can implement a push function that will push the value onto the stack and then update the minimum value, performing comparison functions to see where to store.
#[derive(Debug)]
pub struct SpecialStack<T> {
    data: VecDeque<T>,
    mins: VecDeque<T>,
}

impl<T> SpecialStack<T>
where
    T: Copy + Ord, // or some suitable bounds you want to use
{
    /// Create a new empty SpecialStack.
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            mins: VecDeque::new(),
        }
    }

    /// Push a value onto the stack.
    ///
    /// Requirements / notes:
    /// - O(1) time.
    /// - Must update any tracking needed for get_min().
    ///
    /// ## How min-tracking works
    ///
    /// `mins` is an auxiliary stack that stores the minimum *at the time we pushed*
    /// each new minimum. We only push to `mins` when `value <= current_min` (or when
    /// `mins` is empty). So the back of `mins` is always the current minimum. When we
    /// later pop from `data`, if the popped value equals that back, we pop from `mins`
    /// too, so the previous minimum is restored. Example: push 18,19,29,15,16 →
    /// mins = [18,15]; get_min() = 15; pop twice (16, then 15) → pop mins twice →
    /// mins = [18]; get_min() = 18.
    pub fn push(&mut self, value: T) {
       self.data.push_back(value);
       if self.mins.is_empty() || value <= *self.mins.back().unwrap() {
        self.mins.push_back(value);
       }
    }

    /// Pop the top value from the stack.
    ///
    /// Requirements / notes:
    /// - O(1) time.
    /// - Must keep min tracking in sync: if the value we pop is the current minimum,
    ///   pop it from `mins` too so the previous minimum becomes current.
    pub fn pop(&mut self) -> Option<T> {
        let value = self.data.pop_back()?;
        if self.mins.back() == Some(&value) {
            self.mins.pop_back();
        }
        Some(value)
    }

    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Return the current minimum element in the stack in O(1) time.
    ///
    /// Behavior:
    /// - If the stack is empty you can decide between:
    ///   - returning None, or
    ///   - panicking (document clearly if you choose this).
    pub fn get_min(&self) -> Option<T> {
        self.mins.back().copied()
    }
}

// Optional: place for quick tests / examples.
// You can turn these into proper unit tests if you want.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_usage_from_prompt() {
        let mut s = SpecialStack::new();
        s.push(18);
        s.push(19);
        s.push(29);
        s.push(15);
        s.push(16);
        assert_eq!(s.get_min(), Some(15));
        assert_eq!(s.pop(), Some(16));
        assert_eq!(s.pop(), Some(15));
        assert_eq!(s.get_min(), Some(18));
    }
}

// ---------------------------------------------------------------------------
// Space complexity analysis (notes to write up for recruiter)
// ---------------------------------------------------------------------------
//
// Write your reasoning here:
// - If you use an auxiliary stack (or min-per-element), what extra space
//   do you need relative to the number of stored elements n?
// - Distinguish between:
//   - Space used to store the actual elements (the stack itself).
//   - Extra / auxiliary space used only to support O(1) get_min().
//
// Final space complexity:
// - Total space as a function of n:
// - Auxiliary overhead (beyond storing the elements themselves):

