# Keywords Search Index

Use Cmd/Ctrl+F or grep to find files by topic. Each file lists keywords for problems/solutions it addresses.

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
| `rust_specifics/src/data_collections/arrays_and_hashing/vector/custom_vec.rs` | Vec, CustomVec, Drain, IntoIter, ZST, RawValIter, push, pop, allocation |
| `rust_specifics/src/data_collections/arrays_and_hashing/vector/raw_vec.rs` | RawVec, allocation, deallocation, ZST, heap, Layout |
| `rust_specifics/src/data_collections/arrays_and_hashing/hash_maps.rs` | HashMap, hash table, hashing |
| `rust_specifics/src/data_collections/arrays_and_hashing/hash_sets.rs` | HashSet, set, uniqueness |
| `rust_specifics/src/data_collections/arrays_and_hashing/hash_tables.rs` | hash table, collision, chaining |
| `rust_specifics/src/data_collections/arrays_and_hashing/slices.rs` | slice, [T], array |
| `rust_specifics/src/data_collections/arrays_and_hashing/searching_collections.rs` | search, contains, find |
| `rust_specifics/src/async/*.rs` | async, await, Future, Waker, pinning, lock-free |
| `rust_specifics/src/tokio_specifics/*.rs` | tokio, async runtime, Mutex, channel, spawn, block_on |

## Algorithms

| File | Keywords |
|------|----------|
| `algorithms/src/search_alogrithms/binary_search.rs` | binary search, O(log n), sorted array |
| `algorithms/src/search_alogrithms/pairwise_algorithm.rs` | pairwise, two pointers |
| `algorithms/src/sort_alogrithms/bubble_sort.rs` | bubble sort, O(n^2), in-place |
| `algorithms/src/sort_alogrithms/merge_sort.rs` | merge sort, divide and conquer, O(n log n) |
| `algorithms/src/graph_alogrithms/breadth_first_search.rs` | BFS, graph, queue |
| `algorithms/src/graph_alogrithms/depth_first_search.rs` | DFS, graph, stack, recursion |
| `algorithms/src/graph_alogrithms/djikstra.rs` | Dijkstra, shortest path, weighted graph |
| `algorithms/src/fixed_point_and_floating_point/fixed_point_maths.rs` | fixed point, floating point, precision |

## Data structures

| File | Keywords |
|------|----------|
| `data_structures/src/binary_trees/std_binary_trees.rs` | BTreeMap, BTreeSet, red-black tree |
| `data_structures/src/binary_trees/custom_bt/*.rs` | binary tree, recursion, tree traversal |
| `data_structures/src/simple_structs/vecs_arrays_slices.rs` | Vec, array, slice |
| `data_structures/src/simple_structs/linked_lists.rs` | linked list, Node, next pointer |
| `data_structures/src/simple_structs/stacks.rs` | stack, LIFO, push, pop |
| `data_structures/src/simple_structs/queues.rs` | queue, FIFO |
| `data_structures/src/simple_structs/hash_maps_and_sets.rs` | HashMap, HashSet |
| `data_structures/src/sliding_window.rs` | sliding window, substring, O(n) |
| `data_structures/src/lsm_trees.rs` | LSM tree, log-structured merge |

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
