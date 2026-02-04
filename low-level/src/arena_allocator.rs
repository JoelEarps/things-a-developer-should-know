// =============================================================================
// ARENA ALLOCATOR — Implementation TODO (no logic here, just guidance)
// =============================================================================
//
// WHAT TO DO
// ----------
// - Allocate one big block of memory at creation (e.g. Vec<u8> or raw
//   allocator).
// - alloc(size) (and optionally align): bump a pointer, return pointer/slice
//   into the block. No free of individual allocations.
// - reset(): set bump pointer back to start; all previous allocations are
//   invalid (use-after-reset is undefined behaviour).
// - Drop: free the whole block.
//
// API IDEAS
// ---------
// - Arena::new(capacity) or with_capacity
// - arena.alloc(size: usize) -> Option<*mut u8> or Option<&mut [u8]>
// - arena.reset()
// Optional: arena.alloc_type::<T>(value: T) -> Option<&mut T> (write T, return ref).
//
// DESIGN CHOICES TO DECIDE
// -----------------------
// - Alignment: round up size to align of T if you do typed allocs; support
//   align parameter for alloc(size, align).
// - Bump only: no reuse until reset (simplest). No per-allocation metadata.
// - Capacity: what to do when full — return None, or grow block (then you need
//   to track multiple blocks or realloc).
//
// HOW TO MEASURE PERFORMANCE
// --------------------------
// - Benchmark: many small alloc(size) calls in a loop, then reset, repeat.
// - Compare vs global allocator (e.g. Vec::with_capacity per "allocation" or
//   Box::new) for the same number and size of allocations.
// - Use criterion or std::time::Instant; report time per allocation (or per
//   batch of allocations + reset).
// - Measure: throughput (allocs per second); latency per alloc; compare arena
//   vs malloc/Box. Goal: arena should be much faster (pointer bump vs
//   allocator bookkeeping).
