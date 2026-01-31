<!-- Keywords: interview, HFT, Pulsar, atomic, mutex, polymorphism, map, hash, concurrency, Rust, ITCH, FIX, WebSockets, Drain, IntoIter -->
# HFT Systems Interview Prep

**Role:** [Junior Developer - Front Office](https://pulsar.com/careers/junior-developer-front-office/)  
**Format:** ~2 hours total — 60 min test (40 min technical, 20 min brain teaser) + risk architecture (F2F with Hao)  
**Test style:** Pen and paper, 8 Qs (software + programming), **time complexities expected**

---

## 1. Atomic variables

**What is an atomic variable?**

---

## 2. Mutex

**What is a mutex?**

---

## 3. Atomic vs Mutex

**What is the difference between an atomic and a mutex, and when would you use each?**

---

## 4. Polymorphism

**What is polymorphism?**

---

## 5. Runtime vs compile-time polymorphism

**When is polymorphism executed at runtime and when at compile time?**

---

## 6. Map vs unordered_map (implementation)

**What is the difference between `map` and `unordered_map` in terms of implementation?**

*(C++ `std::map` vs `std::unordered_map`; same ideas apply to Rust `BTreeMap` vs `HashMap`.)*

---

## 7. Map vs unordered_map (complexity)

**What is the search complexity of `map` and of `unordered_map`?**

---

## 8. Block pointers

**What is a block pointer?**

---

## 9. SPL list (singly linked list)

**What is an SPL / singly linked list?** (Structure, operations, typical complexities.)

---

## 10. Hash maps

**How is a hash map implemented? What are the main time complexities?**

---

## 11. Multithreading

**How do you achieve thread-safe shared state? When would you use atomics vs a mutex vs channels?**

---

## 12. Concurrency vs parallelism

**What is the difference between concurrency and parallelism?** Can you have one without the other?

**How would you explain concurrency to someone who thinks it means “running at the same time”?**

**Give an example:** A system that is concurrent but not parallel. One that is parallel but not concurrent. One that is both.

**How does this relate to async/await?** Is async code concurrent? Is it necessarily parallel?

**What is a race condition?** Does parallelism imply race conditions? Does concurrency?

**When would you choose multiple threads (parallelism) vs a single thread with async (concurrency)?** What are the trade-offs?

---

## 13. Time complexities (must know)

**What are the time complexities for:** array/Vec (unsorted and sorted), linked list, hash table, balanced BST/map, binary search, merge sort, quicksort (average)?

---

## 14. Rust-specific

**What are the main benefits of Rust?** (e.g. memory safety, no data races, zero-cost abstractions, performance.)

**What is ownership?** What are the rules (single owner, move semantics, drop)? Why does Rust use it?

**What is borrowing?** What is the difference between `&T` and `&mut T`? What are the borrowing rules (e.g. one mutable ref or many immutable refs, no dangling refs)?

**What is monomorphization?** How does Rust use it for generics, and what are the trade-offs (e.g. compile time, binary size, runtime cost)?

**When would you use generics (`fn f<T: Trait>(x: T)`) vs trait objects (`dyn Trait`)?** What is the runtime cost of each?

**What do `Send` and `Sync` mean?** When is a type `Send`? When is it `Sync`? How does Rust use them for thread safety?

**When would you use `Arc` vs `Rc`?** What about `Mutex<T>` vs `RwLock<T>` for shared mutable state?

**What is the purpose of `Result` and `Option`?** How do they replace null/exceptions and enforce error handling?

**What does “zero-cost abstraction” mean in Rust?** Can you give an example?

---

## 15. Extra topics (from job description)

- **Web:** When would you use REST vs WebSockets?
- **.NET:** Basic C# and async (if they use it for UIs/tooling).

---

## 16. High-frequency / low-latency systems (systems view)

### **Memory & allocation**

- Why avoid heap allocation in the hot path? What is the cost of malloc/free or of growing a `Vec`?
- When would you use object pools, arena allocators, or pre-allocated buffers instead of allocating on demand?
- What is cache-friendly data layout? Why does it matter for latency?
- What is false sharing? How can padding or per-core data reduce it?

### **CPU & cache**

- What is a cache line? How can contention on a single cache line hurt performance across cores?
- Why might you pin threads to specific CPU cores (CPU affinity)? What problem does it solve?
- Why do predictable branches and straight-line code paths help in low-latency code?

What is a cache, what are the access time, what are cache misses
Build your own cache
How to interact with cache's in rust

### **Kernel & syscalls**

- What is the cost of a syscall (context switch, kernel/user boundary)? When should you avoid syscalls in the hot path?
- What is kernel bypass (e.g. DPDK, io_uring)? When would you use it instead of the normal OS network stack?
- When might you prefer busy-wait / polling over blocking (e.g. interrupts) in a latency-critical loop?

### **Concurrency & locking**

- When do lock-free structures (atomics, lock-free queues) beat mutexes in low-latency systems? When might a mutex still be better?
- What is a single-writer / multiple-reader pattern, and why is it useful for latency?
- Why is “minimize time holding a lock” important? What happens if you hold a lock across I/O or across an await?

### **Networking**

- Why might you batch or coalesce small messages? What is the trade-off?
- When would you use UDP instead of TCP for low latency? What do you give up?
- What does “zero-copy” mean in a networking context, and why might it matter?

### **Measurement & observability**

- Why do people care about p99 or p99.9 latency, not just average? What is tail latency and jitter?
- Why might clock source and timestamping matter when measuring sub-millisecond latency?

### **Rust-specific for low latency**

- Why avoid `panic!` or `.unwrap()` in the hot path? What can they do to the call stack and latency?
- When might you use `#[inline]` or avoid dynamic dispatch in a critical path?
- When would you consider `no_std` or `unsafe` for a latency-critical component? What are the trade-offs?

---

## 17. ITCH, FIX and trading protocols

**What is ITCH?** Which exchange(s) use it? What does it carry (e.g. order book updates, trades)?

**What is FIX (Financial Information eXchange)?** What is it used for? Is it human-readable or binary?

**How do ITCH and FIX differ?** (Purpose, format, speed, who uses them.)

**What is the difference between a market data feed and an order entry protocol?** Where do ITCH and FIX fit?

**What might you need to handle when parsing ITCH or FIX messages?** (e.g. binary format, sequence numbers, heartbeats, reconnects.)

---

## 18. WebSockets

**How do WebSockets work?** What happens during the handshake? How do they differ from HTTP?

**Why use WebSockets instead of REST for real-time data (e.g. market data, live updates)?**

**What is the WebSocket handshake?** (HTTP Upgrade request, `101 Switching Protocols`, `Sec-WebSocket-Key`.)

**How are WebSocket frames structured?** (Opcode, payload length, masking — why masking for client→server?)

**What happens when a WebSocket connection drops?** How might you implement reconnection and message replay?

**How do WebSockets relate to TCP?** Are they full-duplex? What about latency and overhead vs raw TCP?

---

## 19. Brain teaser prep (20 min)

- Logical puzzles (e.g. river crossing, weighing, hats).
- Estimation (“How many X in Y?”).
- Short coding on paper (e.g. reverse a linked list, balanced parentheses, find duplicate) — edge cases, loop invariants.

---

## 20. Risk architecture (F2F with Hao)

- How do you think about reliability and failure modes in trading/peripherals?
- How do you avoid single points of failure? How do you monitor and limit risk (e.g. position limits, circuit breakers)?
- “Own your deliverables’ reliability, scalability, and maintainability” — what examples or approach would you give?

---

## Quick revision checklist

- [ ] Atomic: definition, when to use.
- [ ] Mutex: what it is, when to use vs atomics.
- [ ] Polymorphism: static vs dynamic, when each runs.
- [ ] map vs unordered_map: implementation and complexity.
- [ ] Block pointer, SPL list, hash map.
- [ ] Multithreading: Send/Sync, Mutex vs channel vs atomic.
- [ ] Concurrency vs parallelism: definitions, examples, async vs threads.
- [ ] Time complexities for array, list, hash, tree, binary search, sorts.
- [ ] Rust: ownership, borrowing, monomorphization, Send/Sync, generics vs dyn Trait.
- [ ] HFT/low-latency: allocation hot path, cache/false sharing, kernel bypass, lock-free vs mutex, p99 latency.
- [ ] ITCH, FIX: purpose, format, market data vs order entry.
- [ ] WebSockets: handshake, frames, when vs REST, reconnection.
- [ ] One or two “elegant solution to a complex problem” stories.

---

## 21. Sample practice questions (pen-and-paper style)

Use these to simulate the 40‑minute technical test. Write answers by hand and state time complexities.

1. **Implement** (pseudocode or Rust): Insert a node at the head of a singly linked list. State time complexity.
2. **Implement:** Binary search on a sorted array. State time complexity. When would you use it vs linear search?
3. **Explain:** Why might an `unordered_map` lookup be O(n) instead of O(1)?
4. **Explain:** You have a counter shared by 10 threads. Would you use an atomic or a mutex? Why?
5. **Implement:** Reverse a singly linked list in-place. State time and space complexity.
6. **Explain:** What is the difference between `Vec<T>` and `Vec<Box<dyn Trait>>` in terms of when the concrete type is known?
7. **State complexity:** Search in a red-black tree; insert in a hash table (average case); merge sort on n elements.
8. **Design:** You need to store key–value pairs, iterate in sorted order sometimes, and look up by key often. Which data structure(s) and why?

---

Good luck with the interview.
