//! Rust learning crate — organised by domain for a clear learning path.
//! See README.md and docs/RUST_EXPERT_LEARNING_TODO.md for order.

// 1. Fundamentals
pub mod fundamentals;

// 2. Ownership
pub mod ownership;

// 3. Types and traits
pub mod types_and_traits;

// 4. Memory
pub mod memory;

// 5. Collections
pub mod collections;

// 6. Iterators
pub mod iterators;

// 7. Macros
pub mod macros;

// 8. Concurrency (sync + async)
pub mod concurrency;

// 9. Unsafe and FFI
pub mod unsafe_and_ffi;

// 10. Testing
pub mod testing;

// 11. Patterns (feature flags, etc.)
pub mod patterns;

// Re-export for bin and external use
pub use patterns::feature_flags::AppComponentManager;
