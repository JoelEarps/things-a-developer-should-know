# Keywords Search Index

Use Cmd/Ctrl+F or grep to find files by topic. Each file lists keywords for problems/solutions it addresses.

---

## When to use (data structures & algorithms)

Search for a use case to find which structure or algorithm to use.

| Use when / Use case | Data structure or algorithm | File / topic |
|---------------------|-----------------------------|--------------|
| **Need fast lookup by key, no ordering** | HashMap, hash table, unordered map | hash_maps, hash_tables, BTreeMap vs HashMap |
| **Need sorted keys or range queries** ("all keys between A and B") | BTreeMap, ordered map, red-black tree | std_binary_trees, map vs unordered_map |
| **Need uniqueness, no duplicates, fast "contains"** | HashSet | hash_sets, hash_maps_and_sets |
| **Dynamic array, push/pop at end, random access by index** | Vec, vector | vecs_arrays_slices, custom_vec |
| **Queue: FIFO, add at back remove from front** | Queue, VecDeque (ring buffer) | queues |
| **Stack: LIFO, push/pop at one end** | Stack, Vec | stacks |
| **Add/remove at both ends efficiently** | VecDeque (ring buffer), not linked list | queues, VecDeque |
| **O(1) merge two lists, lock-free queue, intrusive list** | Singly linked list | linked_lists, SPL list |
| **Contiguous fixed-size collection** | Array, [T; N], slice | vecs_arrays_slices, slices |
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
| `rust_specifics/src/atomic.rs` | atomic, mutex, lock-free, concurrency, ordering, SeqCst, Relaxed, Acquire, Release |
| `rust_specifics/src/borrowing_and_references.rs` | borrowing, references, mutable reference, ownership, scope |
| `rust_specifics/src/box.rs` | Box, heap allocation, owned pointer |
| `rust_specifics/src/copy_vs_clone.rs` | Copy, Clone, derive, shallow copy, deep copy |
| `rust_specifics/src/dyn_trait.rs` | dyn trait, dynamic dispatch, vtable, polymorphism, fat pointer |
| `rust_specifics/src/lifetimes.rs` | lifetime, lifetime elision, borrow checker |
| `rust_specifics/src/macros.rs` | macro_rules!, macro, metaprogramming |
| `rust_specifics/src/pointers.rs` | raw pointer, fat pointer, thin pointer |
| `rust_specifics/src/streams.rs` | Stream, async iterator |
| `rust_specifics/src/string_and_ampersand_str.rs` | String, str, &str, string slice |
| `rust_specifics/src/unique.rs` | Unique, raw pointer |
| `rust_specifics/src/unsafe_vs_safe.rs` | unsafe, safe Rust, FFI |
| `rust_specifics/src/variables_and_mutability.rs` | mut, const, variable |
| `rust_specifics/src/control_flow.rs` | if, match, loop, for |
| `rust_specifics/src/custom_types.rs` | struct, enum, impl |
| `rust_specifics/src/higher_order_funcs.rs` | iterator, map, filter, closure |
| `rust_specifics/src/combinators/custom_combinator.rs` | combinator, Option, Result, and_then |
| `rust_specifics/src/feature_flags/*.rs` | feature flags, conditional compilation, strategy pattern |
| `rust_specifics/src/testing/*.rs` | testing, mocking, mockall, unit test |
| `rust_specifics/src/memory_management/box_and_pin.rs` | Pin, Unpin, memory layout |
| `rust_specifics/src/data_collections/arrays_and_hashing/vector/custom_vec.rs` | Vec, CustomVec, Drain, IntoIter, ZST | dynamic array, push/pop, random access |
| `rust_specifics/src/data_collections/arrays_and_hashing/vector/raw_vec.rs` | RawVec, allocation, deallocation, ZST, heap, Layout | buffer ownership, grow/shrink |
| `rust_specifics/src/data_collections/arrays_and_hashing/hash_maps.rs` | HashMap, hash table, hashing | fast lookup by key, key-value store |
| `rust_specifics/src/data_collections/arrays_and_hashing/hash_sets.rs` | HashSet, set, uniqueness | no duplicates, O(1) contains |
| `rust_specifics/src/data_collections/arrays_and_hashing/hash_tables.rs` | hash table, collision, chaining | how hashing works, collision handling |
| `rust_specifics/src/data_collections/arrays_and_hashing/slices.rs` | slice, [T], array | view into Vec/array, no copy |
| `rust_specifics/src/data_collections/arrays_and_hashing/searching_collections.rs` | search, contains, find | lookup in collections |
| `rust_specifics/src/async/*.rs` | async, await, Future, Waker, pinning, lock-free |
| `rust_specifics/src/tokio_specifics/*.rs` | tokio, async runtime, Mutex, channel, spawn, block_on |

## Algorithms

| File | Keywords | Use when |
|------|----------|----------|
| `algorithms/src/search_alogrithms/binary_search.rs` | binary search, O(log n), sorted array | sorted data, find element or insertion point |
| `algorithms/src/search_alogrithms/pairwise_algorithm.rs` | pairwise, two pointers | two sum, palindrome, merge two sorted |
| `algorithms/src/sort_alogrithms/bubble_sort.rs` | bubble sort, O(n^2), in-place | teaching, tiny arrays |
| `algorithms/src/sort_alogrithms/merge_sort.rs` | merge sort, divide and conquer, O(n log n) | stable sort, linked list sort, external sort |
| `algorithms/src/graph_alogrithms/breadth_first_search.rs` | BFS, graph, queue | shortest path unweighted, level-order |
| `algorithms/src/graph_alogrithms/depth_first_search.rs` | DFS, graph, stack, recursion | cycle detection, topological sort, backtracking |
| `algorithms/src/graph_alogrithms/djikstra.rs` | Dijkstra, shortest path, weighted graph | weighted shortest path, non-negative edges |
| `algorithms/src/fixed_point_and_floating_point/fixed_point_maths.rs` | fixed point, floating point, precision | deterministic math, finance, embedded |

## Data structures

| File | Keywords | Use when |
|------|----------|----------|
| `data_structures/src/binary_trees/std_binary_trees.rs` | BTreeMap, BTreeSet, red-black tree | sorted keys, range queries, ordered iteration |
| `data_structures/src/binary_trees/custom_bt/*.rs` | binary tree, recursion, tree traversal | hierarchical data, search tree |
| `data_structures/src/simple_structs/vecs_arrays_slices.rs` | Vec, array, slice | dynamic array, random access, push/pop at end |
| `data_structures/src/simple_structs/linked_lists.rs` | linked list, Node, next pointer | O(1) merge/split, lock-free queue, intrusive list |
| `data_structures/src/simple_structs/stacks.rs` | stack, LIFO, push, pop | undo, expression parsing, DFS |
| `data_structures/src/simple_structs/queues.rs` | queue, FIFO | task queue, BFS, producer-consumer |
| `data_structures/src/simple_structs/hash_maps_and_sets.rs` | HashMap, HashSet | fast lookup by key, uniqueness, O(1) contains |
| `data_structures/src/sliding_window.rs` | sliding window, substring, O(n) | max subarray, substring, contiguous window |
| `data_structures/src/lsm_trees.rs` | LSM tree, log-structured merge | write-heavy storage, databases, leveled compaction |

## Hackerrank / Leetcode

| File | Keywords |
|------|----------|
| `hackerrank/src/array_ratio.rs` | array, ratio, positive negative zero |
| `hackerrank/src/comparison_sorting.rs` | sorting, comparison count |
| `hackerrank/src/min_max_sum.rs` | min sum, max sum, 4 of 5 elements |
| `hackerrank/src/intermediate/merge_and_sort_intervals.rs` | merge intervals, overlapping |
| `hackerrank/src/intermediate/sherlock_and_valid_strings.rs` | string, frequency, valid |
| `hackerrank/src/one_hour_tests/write_a_rate_limiter.rs` | rate limiter, token bucket, sliding window |
| `hackerrank/src/one_hour_tests/general_comp_sci/top_k_frequent_elements.rs` | top K, heap, frequency |
| `leetcode_questions/src/median_of_two/*.rs` | median, two sorted arrays, binary search |

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
| `solana/src/bin/fetch_market_data_from_solana.rs` | Solana, market data, RPC |
| `solana/src/bin/favourites_program.rs` | Solana, program |

## Interview prep

| File | Keywords |
|------|----------|
| `HFT-interview-prep.md` | interview, HFT, Pulsar, atomic, mutex, polymorphism, map, hash, concurrency, Rust, ITCH, FIX, WebSockets |
