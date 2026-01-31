// Keywords: atomic, mutex, lock-free, concurrency, ordering, SeqCst, Relaxed, Acquire, Release
/// Atomic operations refer to the smallest programmable unit. In concurrent programming this typically means that once the operation 
/// is started it either finishes (or doesn't) and doesn't become visible to the rest of the program until that happens. This therefore guarantees safety across concurrent threads/ operations.
/// Ordering refers 
/// For rust the ordering enums are available as part of the std lib.
// Relaxed
// Summary: No ordering guarantees—just atomicity.
// Use when: You don’t care about the order of memory operations, just that updates are atomic.
// Avoid when: You need synchronization between threads.

// Release
// Summary: Ensures everything before the store is visible to other threads that do a matching Acquire load.
// Use when: Writing to a shared value and want other threads to see prior changes.
// Avoid when: Reading from shared memory or synchronizing two-way.

// ✅ Acquire
// Summary: Ensures everything after the load happens after the store it reads from.
// Use when: Reading shared values that were stored with Release.
// Avoid when: Writing to shared memory.

// ✅ AcqRel (Acquire + Release)
// Summary: Combines Acquire (on load) and Release (on store).
// Use when: You do both read and write in the same atomic op (e.g. fetch_update, compare_exchange).
// Avoid when: You’re only doing reads or only writes.

// ✅ SeqCst (Sequentially Consistent)
// Summary: Strongest ordering—operations appear in the same order across all threads.
// Use when: You want simplicity and maximum safety.
// Avoid when: You need performance and can tolerate weaker ordering.


/// Show examples for all three