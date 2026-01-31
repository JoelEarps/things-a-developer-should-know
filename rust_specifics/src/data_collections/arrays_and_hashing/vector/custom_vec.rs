// Keywords: Vec, CustomVec, Drain, IntoIter, ZST, RawValIter, push, pop, allocation, iterator
//! CustomVec: dynamic array built on RawVec.
//! Tracks length and handles element operations (push, pop, insert, remove).
//! Uses Deref/DerefMut to delegate to slice for iter, iter_mut, first, last, etc.
//!
//! ## Zero-Sized Types (ZSTs)
//! See: https://doc.rust-lang.org/nomicon/vec/vec-zsts.html
//! For ZSTs: ptr::read/write are no-ops, ptr.add(n) is a no-op. We use usize arithmetic
//! for iterator pointers and read from NonNull::dangling() when yielding ZST values.
//!
//! ## When to use which iterator
//! See: https://doc.rust-lang.org/nomicon/vec/vec-drain.html
//!
//! | Method    | Yields   | Consumes Vec? | Keeps allocation? | Use when |
//! |-----------|----------|---------------|-------------------|----------|
//! | `iter()`  | `&T`     | No            | Yes               | Read-only, Vec stays usable |
//! | `iter_mut()` | `&mut T` | No         | Yes               | Modify in place, Vec stays usable |
//! | `into_iter()` | `T`   | Yes           | No (freed)        | Consume Vec, take ownership of elements |
//! | `drain()` | `T`      | No            | Yes               | Take ownership of elements but keep Vec |
//!
//! **`into_iter()`** — Use when you're done with the Vec and want to consume it (e.g. `for x in vec`).
//! The Vec is dropped after iteration; allocation is freed.
//!
//! **`drain()`** — Use when you want to move elements out but keep the Vec alive and its buffer
//! for reuse. E.g. process a batch of items, then push new ones without reallocating. Common in
//! pooling, batch processing, or when you want to "clear and refill" efficiently.

use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

use super::raw_vec::RawVec;

pub struct CustomVec<T> {
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for CustomVec<T> {}
unsafe impl<T: Sync> Sync for CustomVec<T> {}

impl<T> CustomVec<T> {
    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if mem::size_of::<T>() == 0 {
            // ZST: no allocation needed. ptr::write is a no-op; we just increment len.
            // We never grow (cap is usize::MAX for ZSTs).
            self.len += 1;
            // We must "use" value to run its destructor if it has one. For ZST, drop is usually
            // a no-op, but we can't just ignore it. mem::forget would leak; drop(value) runs it.
            mem::forget(value); // ZST drop is typically no-op; we've "stored" len copies logically
        } else {
            if self.len == self.buf.capacity() {
                self.buf.grow();
            }
            unsafe {
                ptr::write(self.buf.ptr().as_ptr().add(self.len), value);
            }
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        if mem::size_of::<T>() == 0 {
            // ZST: ptr::read from dangling is safe — it's a no-op, we're fabricating the value.
            // All ZST values are identical, so we can "read" from anywhere (or nowhere).
            Some(unsafe { ptr::read(NonNull::<T>::dangling().as_ptr()) })
        } else {
            Some(unsafe { ptr::read(self.buf.ptr().as_ptr().add(self.len)) })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len, "Index out of bounds");
        if mem::size_of::<T>() == 0 {
            self.len += 1;
            mem::forget(value);
        } else {
            if self.len == self.buf.capacity() {
                self.buf.grow();
            }
            unsafe {
                ptr::copy(
                    self.buf.ptr().as_ptr().add(index),
                    self.buf.ptr().as_ptr().add(index + 1),
                    self.len - index,
                );
                ptr::write(self.buf.ptr().as_ptr().add(index), value);
            }
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        self.len -= 1;
        if mem::size_of::<T>() == 0 {
            let result = unsafe { ptr::read(NonNull::<T>::dangling().as_ptr()) };
            return result;
        }
        unsafe {
            let result = ptr::read(self.buf.ptr().as_ptr().add(index));
            ptr::copy(
                self.buf.ptr().as_ptr().add(index + 1),
                self.buf.ptr().as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    /// Drain: borrows self, yields elements by value, leaves allocation intact.
    ///
    /// **Use when**: You want to take ownership of elements but keep the Vec and its buffer.
    /// E.g. process a batch, then push new items without reallocating. Or "clear and refill" a Vec.
    /// Sets len to 0 immediately — if Drain is forgotten, we leak elements but avoid double-drop.
    pub fn drain(&mut self) -> Drain<'_, T> {
        let len = self.len;
        let iter =
            unsafe { RawValIter::new(std::slice::from_raw_parts(self.buf.ptr().as_ptr(), len)) };
        self.len = 0;
        Drain {
            vec: PhantomData,
            iter,
        }
    }
}

impl<T> Drop for CustomVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        if self.len > 0 {
            unsafe {
                if elem_size == 0 {
                    // ZST: we never allocated. Drop each "element" — for ZST this runs drop len times.
                    // The pointer can be dangling; drop_in_place doesn't dereference for ZST.
                    let slice =
                        std::slice::from_raw_parts_mut(NonNull::<T>::dangling().as_ptr(), self.len);
                    ptr::drop_in_place(slice);
                } else {
                    ptr::drop_in_place(std::slice::from_raw_parts_mut(
                        self.buf.ptr().as_ptr(),
                        self.len,
                    ));
                }
            }
        }
        // buf (RawVec) drops here and deallocates the buffer (no-op for ZSTs)
    }
}

impl<T> Deref for CustomVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.buf.ptr().as_ptr(), self.len) }
    }
}

impl<T> DerefMut for CustomVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.buf.ptr().as_ptr(), self.len) }
    }
}

// =============================================================================
// RawValIter: shared pointer-chasing iteration logic for IntoIter and Drain.
// See: https://doc.rust-lang.org/nomicon/vec/vec-drain.html
//
// For sized types: start/end are pointers, we advance with ptr.add(1).
// For ZSTs: ptr.add(n) is a no-op, so start == end always. We use start/end as
// *counters* (cast to usize) and read from NonNull::dangling() when yielding.
// =============================================================================

struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {
    /// # Safety
    /// The slice must be valid for the duration of iteration. RawValIter has no
    /// lifetime — caller ensures the underlying allocation outlives this iterator.
    unsafe fn new(slice: &[T]) -> Self {
        if slice.len() == 0 {
            RawValIter {
                start: slice.as_ptr(),
                end: slice.as_ptr(),
            }
        } else if mem::size_of::<T>() == 0 {
            // ZST: ptr.add(len) is a no-op, so we'd get start == end and iterate never.
            // Use start/end as byte-sized counters: end = start + len (as integers).
            RawValIter {
                start: slice.as_ptr(),
                end: ((slice.as_ptr() as usize) + slice.len()) as *const T,
            }
        } else {
            RawValIter {
                start: slice.as_ptr(),
                end: slice.as_ptr().add(slice.len()),
            }
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    // ZST: advance "counter", yield fabricated value from dangling.
                    // ptr::read(dangling) is safe for ZST — no actual read occurs.
                    self.start = (self.start as usize + 1) as *const T;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.add(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const T;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.sub(1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

// =============================================================================
// IntoIter: consumes CustomVec, yields elements. Owns RawVec for deallocation.
//
// **Use when**: Consuming the Vec entirely (e.g. `for x in vec`). The Vec is moved in,
// iteration yields owned T, and the allocation is freed when the iterator is dropped.
// =============================================================================

pub struct IntoIter<T> {
    _buf: RawVec<T>,
    iter: RawValIter<T>,
}

impl<T> IntoIterator for CustomVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = unsafe { RawValIter::new(&self) };
        // Move buf out without running CustomVec::drop. We can't destructure because
        // CustomVec implements Drop. ptr::read + mem::forget is the standard pattern.
        let buf = unsafe { ptr::read(&self.buf) };
        mem::forget(self);

        IntoIter { _buf: buf, iter }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // Drop any remaining elements (e.g. if iteration was short-circuited).
        for _ in &mut *self {}
        // _buf (RawVec) drops here and deallocates. No-op for ZSTs.
    }
}

// =============================================================================
// Drain: borrows CustomVec, yields elements. Does NOT own allocation.
// See: https://doc.rust-lang.org/nomicon/vec/vec-drain.html
//
// **Use when**: Moving elements out but keeping the Vec for reuse. E.g. batch processing,
// object pools, or clearing without reallocating. Unlike into_iter, the Vec stays alive.
//
// We set len = 0 when creating Drain so the Vec "forgets" its elements.
// If Drain is forgotten, we leak — but we avoid double-drop. When Drain drops,
// it consumes remaining elements. The Vec's allocation stays; we just emptied len.
// =============================================================================

pub struct Drain<'a, T> {
    vec: PhantomData<&'a mut CustomVec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::marker::PhantomData;

    use super::*;

    #[test]
    fn custom_vec_demo() {}

    #[test]
    fn regular_vec_demo() {
        let _vec = vec![1, 2, 3, 4, 5];
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
    }

    #[test]
    fn vec_deque_demo() {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);
        deque.push_front(4);
        deque.push_front(5);
    }

    #[test]
    fn cool_combinators() {}

    #[test]
    fn custom_vec_impl() {
        let mut custom_vec: CustomVec<&str> = CustomVec::new();
        custom_vec.push("Hello");
        custom_vec.push("World");
        assert_eq!(custom_vec.len(), 2);
        assert_eq!(custom_vec.pop(), Some("World"));
        assert_eq!(custom_vec.pop(), Some("Hello"));
        assert_eq!(custom_vec.pop(), None);
        assert_eq!(custom_vec.len(), 0);
    }

    /// ZST: CustomVec<()> — unit type has size 0.
    #[test]
    fn custom_vec_zst() {
        let mut v: CustomVec<()> = CustomVec::new();
        v.push(());
        v.push(());
        v.push(());
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.len(), 1);
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), None);
    }

    /// ZST: CustomVec<PhantomData<T>>.
    #[test]
    fn custom_vec_zst_phantom() {
        let mut v: CustomVec<PhantomData<i32>> = CustomVec::new();
        v.push(PhantomData);
        v.push(PhantomData);
        assert_eq!(v.len(), 2);
        let drained: Vec<_> = v.drain().collect();
        assert_eq!(drained.len(), 2);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn custom_vec_into_iter() {
        let mut v: CustomVec<i32> = CustomVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        let collected: Vec<_> = v.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn custom_vec_drain() {
        let mut v: CustomVec<i32> = CustomVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        let drained: Vec<_> = v.drain().collect();
        assert_eq!(drained, vec![1, 2, 3]);
        assert_eq!(v.len(), 0);
        // Vec still usable
        v.push(4);
        assert_eq!(v.len(), 1);
        assert_eq!(v.pop(), Some(4));
    }
}
