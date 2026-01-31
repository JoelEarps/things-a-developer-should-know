// Keywords: RawVec, allocation, deallocation, ZST, heap, Layout, NonNull
//! RawVec: owns the heap buffer (ptr + capacity) and handles allocation/deallocation.
//! Does NOT track length or drop elements — that is CustomVec's responsibility.
//! Used to deduplicate allocation logic between CustomVec and IntoIter.
//!
//! ## Zero-Sized Types (ZSTs)
//! See: https://doc.rust-lang.org/nomicon/vec/vec-zsts.html
//!
//! **What are ZSTs?** Types where `size_of::<T>() == 0`. Examples: `()`, `PhantomData<T>`,
//! `[T; 0]`, and empty enums. They take no memory — all values are indistinguishable at runtime.
//!
//! **Why special handling?**
//! 1. **Allocator API**: `alloc(0)` is undefined behavior. We must never allocate for ZSTs.
//! 2. **Pointer offsets**: `ptr.add(n)` is a no-op (0 bytes per element), so pointer arithmetic
//!    breaks. We use `usize` arithmetic for iterators.
//! 3. **Deallocation**: Deallocating a ZST buffer would be freeing something we never allocated.
//!
//! **When are ZSTs used?**
//! - `PhantomData<T>`: Mark structs as "containing" T for variance/ownership without storing it.
//! - `()`: Unit type; used as "no meaningful value" (e.g. `Vec<()>` as a compact set of indices).
//! - Marker types: Type-level flags with no runtime representation.
//! - Zero-length arrays: `[T; 0]` for generic code that must work with any `N`.
//!
//! For ZSTs we use `NonNull::dangling()` as our "allocation" and `cap = usize::MAX` so we never grow.

use std::alloc::{self, Layout};
use std::mem;
use std::ptr::NonNull;

pub struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    /// Create a new RawVec. For ZSTs, we use `cap = usize::MAX` and never allocate.
    /// `NonNull::dangling()` serves as both "unallocated" and "ZST allocation".
    pub fn new() -> Self {
        // ZSTs: cap = usize::MAX so we never need to grow. Dangling ptr is "valid" for ZSTs.
        // Sized types: cap = 0, no allocation yet.
        let cap = if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            0
        };

        RawVec {
            ptr: NonNull::dangling(),
            cap,
        }
    }

    /// Capacity of the allocated buffer (number of T slots).
    /// For ZSTs this is `usize::MAX` (effectively infinite).
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Raw pointer to the buffer. Valid for indices 0..capacity when cap > 0.
    /// For ZSTs with cap = usize::MAX, this is dangling — that's OK; we never dereference it.
    pub fn ptr(&self) -> NonNull<T> {
        self.ptr
    }

    /// Grow the buffer: double capacity or allocate 1 if empty.
    /// For ZSTs this panics — we should never need to grow (cap is already usize::MAX).
    pub fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();
        // Only deallocate if we actually allocated. ZSTs: we never allocated; sized types: check cap.
        if self.cap != 0 && elem_size != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}
