// =============================================================================
// OBJECT POOL — Implementation TODO (no logic here, just guidance)
// =============================================================================
//
// WHAT TO DO
// ----------
// - Pre-allocate a fixed set of objects (e.g. Vec<T> or array of T).
// - When something needs an object: take one from the pool (e.g. pop from a
//   Vec of available indices, or swap with a "in use" bitmap).
// - When done: return the object to the pool (e.g. push index back, or clear
//   "in use").
// - No malloc/free in the hot path — only pool get/return.
//
// API IDEAS
// ---------
// - Pool::new(capacity) or with_capacity
// - pool.get() -> Option<T>   (take from pool; T: Default or reset state)
// - pool.return_obj(obj: T)   (put back)
// Or: pool.get() -> Option<PoolGuard<T>> where Drop on Guard returns to pool.
//
// DESIGN CHOICES TO DECIDE
// -----------------------
// - Type: Generic T? T: Default? Or a Reset trait? Or wrapper struct?
// - Thread safety: single-threaded pool vs Mutex<Pool> vs lock-free queue.
// - Backing storage: Vec<T>, or Vec<Option<T>>, or raw buffer + indices.
//
// HOW TO MEASURE PERFORMANCE
// --------------------------
// - Benchmark: hot path that does N iterations of get() + use + return_obj().
// - Compare vs same workload using Vec::new() / drop every time (i.e. malloc/free
//   or Vec allocation per object).
// - Use criterion (cargo add criterion; benchmark in benches/) or
//   std::time::Instant in a loop with many iterations; report mean/p99 latency.
// - Measure: time per get+return cycle (nanoseconds); variance (jitter).
// - Goal: pool path should be much faster and more stable than allocator path.
