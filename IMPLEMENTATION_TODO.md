# Implementation TODO — everything left to implement

Categorised by topic. Tick items as you complete them.

---

## Fundamentals (data structures & algorithms)

- [ ] **Min heap / priority queue**
  - Understand heap; min vs max; when to use (Top-K, Dijkstra).
  - Use or implement in Rust (`std::collections::BinaryHeap` or custom).
  - Time complexity: O(log n) insert, O(1) peek, O(log n) pop.

- [ ] **Buckets / bucket sort**
  - Bucket sort; when to use (e.g. frequencies with known range).
  - Space–time tradeoffs.

- [ ] **Sliding window**
  - Practice at least one sliding window problem.
  - Link hackerrank questions to `data_structures/src/sliding_window.rs` (see TODOs there).
  - Implement or complete stubs in `data_structures/src/sliding_window.rs` (replace `todo!()`).

- [ ] **Fixed window**
  - One fixed-window problem; compare with sliding window.

- [ ] **Time complexity examples** (`algorithms/src/time_complexity_explained/`)
  - [ ] O(1) — `constant.rs`: add small example + how to measure.
  - [ ] O(log n) — `logarithmic.rs`
  - [ ] O(n) — `linear.rs`
  - [ ] O(n log n) — `linearithmic.rs`
  - [ ] O(n²) — `quadratic.rs`
  - [ ] O(n^k) — `polynomial.rs`
  - [ ] O(2^n) — `exponential.rs`
  - [ ] O(n!) — `factorial.rs`

- [ ] **Binary search**
  - Raw binary search implementation note in `algorithms/src/search_alogrithms/binary_search.rs`.

- [ ] **HashMap iteration**
  - `into_iter()` vs `iter()` vs `iter_mut()`; ownership; HashMap → Vec of pairs.

- [ ] **Related (future)**
  - Quickselect (O(n) average Top-K); heap optimisation for Top-K; counting sort; HashMap entry API.

---

## Rust (language, std, patterns)

- [ ] **Custom HashSet / HashMap**
  - `rust_specifics/.../hash_structures/hashsets/`: “Implementing my own hashset — what functions do we need?”
  - `rust_specifics/.../hash_structures/hashmaps/custom_hashmap.rs`: implement or add notes.

- [ ] **Pin**
  - `rust_specifics/src/async_rust/pin.rs`: complete TODO “Show this here”.

- [ ] **Copy vs Clone**
  - `rust_specifics/src/copy_vs_clone.rs`: try uncommenting the TODO lines and explain.

- [ ] **Performance optimisation in Rust**
  - `rust_specifics/src/performance_optimisation_in_rust/ReadMe.md`: item 5 — “Platform-Specific Implementations with cfg”.

- [ ] **Comparison sorting**
  - `hackerrank/src/comparison_sorting.rs`: add proper type casting checks (replace panic/code smell).

- [ ] **Fixed-point maths**
  - `algorithms/src/fixed_point_and_floating_point/fixed_point_maths.rs`: generic/templated version (TODO in file).

- [ ] **Array ratio**
  - `hackerrank/src/array_ratio.rs`: implement display for custom type with ToString.

---

## High-performance / low-latency

- [ ] **Object pool**
  - `low-level/src/object_pool.rs`: implement from comment boilerplate; measure vs allocator (see file for how to measure).

- [ ] **Arena allocator**
  - `low-level/src/arena_allocator.rs`: implement from comment boilerplate; measure vs global allocator.

- [ ] **Cache-friendly layout**
  - Add a small example (e.g. struct layout, iteration order) and optional benchmark; or document in `low-level/` or `rust_specifics/performance_optimisation_in_rust/`.

- [ ] **False sharing / padding**
  - Add example: per-core counters with padding or `#[repr(align(64))]`; optional benchmark; or document.

- [ ] **Circular buffer**
  - `low-level/readme.md`: implement a circular buffer (e.g. in `low-level/src/`).

- [ ] **Simple malloc/free**
  - `low-level/readme.md`: implement your own simple allocator.

- [ ] **memcpy / strlen**
  - `low-level/readme.md`: implement from scratch (e.g. in C or Rust).

- [ ] **mmap**
  - `low-level/readme.md`: Rust program that maps a file into memory (mmap).

- [ ] **LRU cache**
  - `low-level/readme.md`: “Build your own LRU Cache” (link to resources there).

- [ ] **Kernel bypass / io_uring**
  - `low-level/src/kernel_bypass.rs`, `iou_ring.rs`: implement or expand examples if currently stubs.

---

## Interview prep (Pulsar / HFT)

- [ ] **Trading bot risk at each stage**
  - Fill in `trading-bot-risk-prep.md`: risks and checks for each stage (data, signal, pre-trade, order send, fill, ongoing); “if something goes wrong” section.

- [ ] **Pen-and-paper practice** (`HFT-interview-prep.md` §21)
  - [ ] Implement: insert node at head of singly linked list; state complexity.
  - [ ] Implement: binary search on sorted array; state complexity.
  - [ ] Implement: reverse singly linked list in-place; state time and space complexity.
  - [ ] Explain + design questions (unordered_map O(n), atomic vs mutex, Vec vs Vec<Box<dyn Trait>>, complexities, key–value structure choice).

- [ ] **HFT prep doc — open items**
  - Add answers or notes for: “Garbage Collectors — what are they, why doesn’t Rust need them?”; “Stack vs Heap — when do you allocate, how?” (in `HFT-interview-prep.md`).

---

## Other / learning

- [ ] **One-hour test: Top K frequent elements**
  - `hackerrank/src/one_hour_tests/general_comp_sci/top_k_frequent_elements.rs`: add clarifying questions, assumptions, example; implement solution.

- [ ] **Caching with Rust**
  - Watch / use resources in `low-level/readme.md` (caching videos, LRU cache).

---

## Quick reference — files to touch

| Topic              | File(s) |
|--------------------|---------|
| Object pool        | `low-level/src/object_pool.rs` |
| Arena allocator    | `low-level/src/arena_allocator.rs` |
| Time complexity    | `algorithms/src/time_complexity_explained/*.rs` |
| Trading bot risk   | `trading-bot-risk-prep.md` |
| Sliding window     | `data_structures/src/sliding_window.rs` |
| Hash structures    | `rust_specifics/.../hash_structures/` |
| Pin                | `rust_specifics/src/async_rust/pin.rs` |
| Low-level tasks    | `low-level/readme.md` (list) |
