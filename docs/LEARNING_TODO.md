# Learning TODO List

**See also:** [IMPLEMENTATION_TODO.md](./IMPLEMENTATION_TODO.md) — consolidated list of everything left to implement, categorised (Fundamentals, Rust, High-performance, Interview prep, Other).

---

## Data Structures & Algorithms

- [ ] **Min Heap** / Priority Queue
  - Understanding heap data structure
  - Min heap vs Max heap
  - When to use heaps (e.g., Top-K problems, Dijkstra's algorithm)
  - Implementation in Rust (std::collections::BinaryHeap)
  - Time complexity: O(log n) insert, O(1) peek, O(log n) pop

- [ ] **Buckets** / Bucket Sort
  - Bucket sort algorithm
  - When to use buckets (e.g., counting frequencies with known range)
  - Bucket-based approaches for Top-K problems
  - Space-time tradeoffs

- [ ] **Sliding Window** - 1 Question
  - Practice one sliding window problem
  - Review variable-size vs fixed-size windows
  - When to use sliding window pattern

- [ ] **Fixed Window Notes** - 1 Question
  - Practice one fixed window problem
  - Compare fixed window vs sliding window
  - Use cases and tradeoffs

- [ ] **HashMap into_iter()**
  - Understanding HashMap iteration methods
  - `into_iter()` vs `iter()` vs `iter_mut()`
  - Converting HashMap to Vec of pairs
  - Ownership semantics

## Related Topics (Future Learning)

- [ ] Quickselect algorithm (O(n) average for Top-K)
- [ ] Heap optimization for Top-K problems (O(n log k) vs O(n log n))
- [ ] Counting sort for frequency problems
- [ ] Advanced HashMap operations (entry API patterns)
