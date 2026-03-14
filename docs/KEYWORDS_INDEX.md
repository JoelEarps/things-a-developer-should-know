# Keywords Search Index

Use Cmd/Ctrl+F or grep to find files by topic. Each file lists keywords for problems/solutions it addresses.

---

## When to use (data structures & algorithms)

Search for a use case to find which structure or algorithm to use.

| Use when / Use case | Data structure or algorithm | File / topic |
|---------------------|-----------------------------|--------------|
| **Need fast lookup by key, no ordering** | HashMap, hash table, unordered map | hash_maps, hash_tables, BTreeMap vs HashMap |
| **Need sorted keys or range queries** ("all keys between A and B") | BTreeMap, ordered map, red-black tree | std_binary_trees, map vs unordered_map |
| **Need uniqueness, no duplicates, fast "contains"** | HashSet | hash_structures/hashsets, rust collections |
| **Dynamic array, push/pop at end, random access by index** | Vec, vector | custom_vec, slices |
| **Queue: FIFO, add at back remove from front** | Queue, VecDeque (ring buffer) | data_structures, rust |
| **Stack: LIFO, push/pop at one end** | Stack, Vec | special_stack, data_structures |
| **Add/remove at both ends efficiently** | VecDeque (ring buffer), not linked list | queues, VecDeque |
| **O(1) merge two lists, lock-free queue, intrusive list** | Singly linked list | data_structures |
| **Contiguous fixed-size collection** | Array, [T; N], slice | slices, rust/data_collections |
| **Subarray/substring problems, max sum in window** | Sliding window | sliding_window |
| **Sorted data, find element or insertion point** | Binary search | binary_search |
| **Two pointers moving toward each other or same direction** | Two pointers, pairwise | pairwise_algorithm |
| **Shortest path in weighted graph** | Dijkstra | djikstra |
| **Explore graph level by level, shortest path unweighted** | BFS, queue | breadth_first_search |
| **Explore graph depth-first, cycle detection, backtracking** | DFS, stack or recursion | depth_first_search |
| **Top K frequent elements** | Heap (priority queue), or bucket sort | top_k_frequent_elements |
| **Merge overlapping intervals** | Sort then merge | merge_and_sort_intervals |
| **Rate limiting, throttle requests** | Rate limiter, token bucket, sliding window | write_a_rate_limiter |
| **State machine, illegal states unrepresentable** | Typestate | typestate |
| **High-throughput I/O, avoid syscall overhead** | io_uring, kernel bypass | iou_ring, kernel_bypass |
| **Shared counter or flag across threads, no lock** | Atomic | atomic |
| **Protect shared data, one writer or multiple readers** | Mutex, RwLock | atomic vs mutex, HFT-interview-prep |
| **Types unknown until runtime, heterogeneous collection** | dyn Trait, trait object | dyn_trait |
| **Types known at compile time, zero-cost abstraction** | Generics, monomorphization | dyn_trait, polymorphism |

---

## Rust specifics

| File | Keywords |
|------|----------|
| `rust/src/atomic.rs` | atomic, mutex, lock-free, concurrency, ordering, SeqCst, Relaxed, Acquire, Release |
| `rust/src/borrowing_and_references.rs` | borrowing, references, mutable reference, ownership, scope |
| `rust/src/box.rs` | Box, heap allocation, owned pointer |
| `rust/src/copy_vs_clone.rs` | Copy, Clone, derive, shallow copy, deep copy |
| `rust/src/dyn_trait.rs` | dyn trait, dynamic dispatch, vtable, polymorphism, fat pointer |
| `rust/src/lifetimes.rs` | lifetime, lifetime elision, borrow checker |
| `rust/src/macros.rs` | macro_rules!, macro, metaprogramming |
| `rust/src/pointers.rs` | raw pointer, fat pointer, thin pointer |
| `rust/src/streams.rs` | Stream, async iterator |
| `rust/src/string_and_ampersand_str.rs` | String, str, &str, string slice |
| `rust/src/unique.rs` | Unique, raw pointer |
| `rust/src/unsafe_vs_safe.rs` | unsafe, safe Rust, FFI |
| `rust/src/variables_and_mutability.rs` | mut, const, variable |
| `rust/src/control_flow.rs` | if, match, loop, for |
| `rust/src/custom_types.rs` | struct, enum, impl |
| `rust/src/higher_order_funcs.rs` | iterator, map, filter, closure |
| `rust/src/combinators/custom_combinator.rs` | combinator, Option, Result, and_then |
| `rust/src/feature_flags/*.rs` | feature flags, conditional compilation, strategy pattern |
| `rust/src/testing/*.rs` | testing, mocking, mockall, unit test |
| `rust/src/memory_management/box_and_pin.rs` | Pin, Unpin, memory layout |
| `rust/src/data_collections/arrays_and_hashing/vector/custom_vec.rs` | Vec, CustomVec, Drain, IntoIter, ZST | dynamic array, push/pop, random access |
| `rust/src/data_collections/arrays_and_hashing/vector/raw_vec.rs` | RawVec, allocation, deallocation, ZST, heap, Layout | buffer ownership, grow/shrink |
| `rust/src/data_collections/arrays_and_hashing/hash_structures/hashmaps/` | HashMap, hash table, hashing | fast lookup by key, key-value store |
| `rust/src/data_collections/arrays_and_hashing/hash_structures/hashsets/` | HashSet, set, uniqueness | no duplicates, O(1) contains |
| `rust/src/data_collections/arrays_and_hashing/slices.rs` | slice, [T], array | view into Vec/array, no copy |
| `rust/src/data_collections/arrays_and_hashing/searching_collections.rs` | search, contains, find | lookup in collections |
| `rust/src/async_rust/*.rs` | async, await, Future, Waker, pinning, lock-free |
| `rust/src/tokio_specifics/*.rs` | tokio, async runtime, Mutex, channel, spawn, block_on |

## Algorithms

| File | Keywords | Use when |
|------|----------|----------|
| `algorithms/src/search_algorithms/binary_search.rs` | binary search, O(log n), sorted array | sorted data, find element or insertion point |
| `algorithms/src/search_algorithms/pairwise_algorithm.rs` | pairwise, two pointers | two sum, palindrome, merge two sorted |
| `algorithms/src/sort_algorithms/bubble_sort.rs` | bubble sort, O(n^2), in-place | teaching, tiny arrays |
| `algorithms/src/sort_algorithms/merge_sort.rs` | merge sort, divide and conquer, O(n log n) | stable sort, linked list sort, external sort |
| `algorithms/src/graph_algorithms/breadth_first_search.rs` | BFS, graph, queue | shortest path unweighted, level-order |
| `algorithms/src/graph_algorithms/depth_first_search.rs` | DFS, graph, stack, recursion | cycle detection, topological sort, backtracking |
| `algorithms/src/graph_algorithms/djikstra.rs` | Dijkstra, shortest path, weighted graph | weighted shortest path, non-negative edges |
| `algorithms/src/fixed_point_and_floating_point/fixed_point_maths.rs` | fixed point, floating point, precision | deterministic math, finance, embedded |

## Data structures

| File | Keywords | Use when |
|------|----------|----------|
| `data_structures/src/binary_trees/std_binary_trees.rs` | BTreeMap, BTreeSet, red-black tree | sorted keys, range queries, ordered iteration |
| `data_structures/src/binary_trees/custom_bt/*.rs` | binary tree, recursion, tree traversal | hierarchical data, search tree |
| `data_structures/src/simple_structs/special_stack.rs` | stack, LIFO, special stack | undo, expression parsing |
| `data_structures/src/sliding_window.rs` | sliding window, substring, O(n) | max subarray, substring, contiguous window |
| `data_structures/src/lsm_trees.rs` | LSM tree, log-structured merge | write-heavy storage, databases, leveled compaction |

## Practice (LeetCode / HackerRank)

| File | Keywords |
|------|----------|
| `practice/src/hackerrank/array_ratio.rs` | array, ratio, positive negative zero |
| `practice/src/hackerrank/comparison_sorting.rs` | sorting, comparison count |
| `practice/src/hackerrank/min_max_sum.rs` | min sum, max sum, 4 of 5 elements |
| `practice/src/hackerrank/intermediate/merge_and_sort_intervals.rs` | merge intervals, overlapping |
| `practice/src/hackerrank/intermediate/sherlock_and_valid_strings.rs` | string, frequency, valid |
| `practice/src/hackerrank/one_hour_tests/write_a_rate_limiter.rs` | rate limiter, token bucket, sliding window |
| `practice/src/hackerrank/one_hour_tests/general_comp_sci/top_k_frequent_elements.rs` | top K, heap, frequency |
| `practice/src/leetcode/pareto_set/arrays_and_hashing/` | LeetCode arrays & hashing, find duplicates |
| `practice/src/leetcode/median_of_two/` | median, two sorted arrays, binary search |

## Design patterns

| File | Keywords |
|------|----------|
| `design_patterns/src/state_machines/typestate.rs` | typestate, state machine, compile-time states |

## Low-level / systems

| File | Keywords |
|------|----------|
| `low-level/src/iou_ring.rs` | io_uring, async I/O, kernel |
| `low-level/src/kernel_bypass.rs` | kernel bypass, DPDK, zero-copy |

## Solana / blockchain

| File | Keywords |
|------|----------|
| `projects/solana/src/bin/fetch_market_data_from_solana.rs` | Solana, market data, RPC |
| `projects/solana/src/bin/favourites_program.rs` | Solana, program |

## Interview prep

| File | Keywords |
|------|----------|
| `docs/HFT-interview-prep.md` | interview, HFT, Pulsar, atomic, mutex, polymorphism, map, hash, concurrency, Rust, ITCH, FIX, WebSockets |
