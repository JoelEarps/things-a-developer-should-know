<!-- Keywords: interview, HFT, Pulsar, atomic, mutex, polymorphism, map, hash, concurrency, Rust, ITCH, FIX, WebSockets, Drain, IntoIter -->
# HFT Systems Interview Prep

**Role:** [Junior Developer - Front Office](https://pulsar.com/careers/junior-developer-front-office/)  
**Format:** ~2 hours total — 60 min test (40 min technical, 20 min brain teaser) + risk architecture (F2F with Hao)  
**Test style:** Pen and paper, 8 Qs (software + programming), **time complexities expected**

---

## 1. Atomic variables

**What is an atomic variable?**

An atomic operation is one the processor cannot interrupt. An atomic variable's operations are indivisible — no other thread can observe a half-updated value.

**Properties:**

- **Lock-free** — use low-level atomic machine instructions (e.g. compare-and-swap) for thread safety with no lock overhead.
- **Non-blocking** — no locks, so no deadlocks.
- **Single variable only** — atomics protect one scalar (counter, flag, pointer). For multiple related values, use a mutex.

**Compare-and-swap (CAS):** Hardware-supported instruction:

1. Read the current value of the memory location
2. Compare with expected value
3. If equal, write new value; otherwise fail (caller can retry)

If two threads CAS the same location, one succeeds and one fails (and can retry). Modern CPUs: `CMPXCHG` on x86.

**Rust:** `AtomicU32`, `AtomicUsize`, `AtomicBool`. Operations: `load`, `store`, `fetch_add`, `compare_exchange`. Memory ordering (Relaxed, SeqCst, Acquire, Release) controls visibility of other reads/writes.

**Use when:** Single shared variable (counter, flag), lock-free structures, or high contention where mutex overhead would hurt.

---

## 2. Mutex

**What is a mutex?**

Concurrent programming requires synchronisation. A data race occurs when threads access shared data simultaneously. A **Mutex** (MUTual EXclusion) ensures only one thread can hold the lock at a time.

**How it works:**

- Fundamentally a flag in memory (0 = unlocked, 1 = locked). Operations: **lock** and **unlock**.
- Thread must **lock** before accessing protected data, then **unlock** when done. Only one owner at a time.
- Lock/unlock use atomic instructions (test-and-set or compare-and-swap). If lock fails, the thread waits — typically via OS scheduler (blocking) or spinlock (busy-wait).

**Contention:** When another thread tries to lock an already-locked mutex. Locking is cheap; contention causes waiting and overhead. Keep critical sections short and contention low.

**Scope:** Can protect **arbitrary data** (structs, collections, multiple fields) — not limited to a single scalar like atomics.

**Rust types:**

1. **Mutex\<T\>** — exclusive access (one reader or writer)
2. **RwLock\<T\>** — multiple readers OR single writer (better when reads >> writes)
3. **Spinlock** — busy-wait instead of blocking (use when hold time is very short)

**Rust:** `std::sync::Mutex<T>` + `Arc<Mutex<T>>` for shared ownership across threads.

**Async note:** Short critical sections (no `.await` inside lock) → `std::sync::Mutex`. Holding lock across `.await` → `tokio::sync::Mutex` or redesign to avoid blocking the runtime.

---

## 3. Atomic vs Mutex

**What is the difference between an atomic and a mutex, and when would you use each?**

| | **Atomic** | **Mutex** |
|---|------------|-----------|
| **Scope** | Single variable (counter, flag, pointer) | Arbitrary data (structs, collections, multiple fields) |
| **Mechanism** | Lock-free, hardware atomics (CAS) | Lock; only one holder at a time |
| **Blocking** | No blocking | Threads block waiting for the lock |
| **Overhead** | Low (few instructions) | Higher (lock/unlock, possible contention) |

**Use atomics when:**

- Single shared variable (counter, flag)
- Lock-free algorithms; high contention where mutex would bottleneck
- Multi-core systems with many threads sharing simple state

**Use a mutex when:**

- Multiple related values must be updated together (e.g. two fields, or search-then-insert)
- Complex critical section; atomic can't express the "transaction"

**Rule of thumb:** Atomic for one variable; mutex for multiple or complex state. Benchmark to confirm.

---

## 4. Polymorphism

**What is polymorphism?**

Same interface, different implementations — objects of different types can be grouped or treated as one supertype based on a shared interface (traits in Rust).

**Rust has two forms:**

1. **Static (compile-time):** Generics with trait bounds — `fn f<T: Trait>(x: T)`. Type-checked at compile time; compiler knows concrete type at each call site.
2. **Dynamic (runtime):** `dyn Trait` — fat pointer (data + vtable). Any type implementing the trait can be passed; which impl runs is decided at runtime. Common use: `dyn Error` for error returns.

---

## 5. Runtime vs compile-time polymorphism

**When is polymorphism executed at runtime and when at compile time?**

| | **Compile-time (generics)** | **Runtime (`dyn Trait`)** |
|---|-----------------------------|---------------------------|
| **When** | Concrete type known at compile time | Concrete type only known at runtime |
| **How** | Monomorphization — compiler generates one version per type | Fat pointer + vtable; indirect call at runtime |
| **Cost** | No runtime overhead; direct calls. Larger binary, slower compile | Indirect call + vtable lookup; smaller binary |
| **Use when** | Types known at compile time; prefer for performance | Heterogeneous collections, API hiding, types from config/user input |

**Rule of thumb:** Use generics when you can; use `dyn Trait` when you need flexibility (e.g. `Vec<Box<dyn Error>>`, trait objects at API boundaries).

---

## 6. Map vs unordered_map (implementation)

**What is the difference between `map` and `unordered_map` in terms of implementation?**

*(C++ `std::map` vs `std::unordered_map`; same ideas apply to Rust `BTreeMap` vs `HashMap`.)*

A **map** is a key–value store (dictionary / associative array) with insert, delete, lookup, and iterate. The difference between ordered and unordered is whether the structure maintains a global ordering of keys.

**Unordered map** (e.g. `std::unordered_map`, Rust `HashMap`):

- **Implementation:** Hash table. Lookup: key → hash function → bucket (chaining or open addressing).
- **Complexity:** O(1) average for lookup, insert, delete. O(n) worst case with bad hashing or many collisions. No ordering: no range queries, no ordered iteration.
- **Rust:** `HashMap` uses a SwissTable-style design with SIMD-friendly lookup and efficient probing.

**Ordered map** (e.g. `std::map`, Rust `BTreeMap`):

- **Implementation:** Balanced search tree (e.g. red-black tree; Rust uses a B-tree). Keys ordered by a comparison rule.
- **Complexity:** O(log n) for lookup, insert, delete. Enables range queries, in-order or reverse traversal.
- **Rust:** `BTreeMap`; alternatives include `IndexMap` (insertion order), Crossbeam structures.

---

## 7. Map vs unordered_map (complexity)

**What is the search complexity of `map` and of `unordered_map`?**

Unordered Map:

Look up - O(1) where O(n) if many collisions.

Ordered map: O(log(n))

---

## 8. Block pointers

**What is a block pointer?**

Block pointers generally refer to a pointer that points to a specific block of memory. They are generally located in high speed memory areas e.g. the stack, the heap. In file systems like ZFS they are embedded within meta data structures e.g. dnodes that point ot data blocks on disk.

In rust there are 4 types:

1. `Box<T>` - A heap allocation for variable of type T.
2. Slices or references to vecs.
3. Raw pointers.
4. Memory allocations using Layout and alloc e.g. when you grow a vec.

---

## 9. SPL list (singly linked list)

**What is an SPL / singly linked list?** (Structure, operations, typical complexities.)

**SPL** here means **singly linked list**: a linear data structure where each **node** holds a value and a **pointer to the next node**. The last node’s pointer is null/None. There is no backward link (unlike a doubly linked list).

**Structure:**

- Node: `(value, next)` where `next` is a pointer to the next node.
- Head pointer: points to the first node. Optional tail pointer for O(1) append.
- Traversal: one direction only (head → … → tail).

**Operations and typical complexity:**

| Operation | Complexity | Notes |
|-----------|------------|--------|
| Access k-th element | O(k) | Must walk from head. No random access. |
| Search (by value) | O(n) | Linear scan. |
| Insert at head | O(1) | New node → head; update head. |
| Insert at tail | O(1) if tail ptr, else O(n) | With tail pointer: link last → new, update tail. |
| Delete at head | O(1) | Move head to head→next; free old head. |
| Delete node (given pointer to it) | O(1) if you have prev, else O(n) | Singly linked: need predecessor to relink; finding it is O(n) unless you already have it. |

**Use when:** You need O(1) insert/delete at one end (e.g. head for a stack or queue front), or when you merge/split lists by rewiring pointers. In Rust, `Vec` is usually preferred; linked lists are awkward due to ownership (each node has one owner).

**Use cases:**

- **Lock-free / concurrent queues** — enqueue allocates a new node and links it; no shared buffer to resize. SPSC/MPSC queues, work-stealing dequeues.
- **Merge two lists in O(1)** — rewire one list’s tail to the other’s head (e.g. merge sort on linked lists, chunked streams).
- **Intrusive lists** — next pointer lives inside the object; unlink by pointer in O(1). Kernel task lists, LRU lists, custom allocator free lists.
- **Functional / persistent “cons” lists** — add at head, share tail; immutable structures.
- **Undo history** — newest at head, walk back through previous states (less common; often a stack/Vec instead).

---

## 10. Hash maps

**How is a hash map implemented? What are the main time complexities?**

A **hash map** is a data structure that stores key-value pairs and provides fast lookup, insertion, and deletion.

**Implementation:**

- Uses a **hash table** — an array of buckets
- A **hash function** converts each key into a hash code (integer)
- The hash code determines which bucket the entry goes into (usually `hash % bucket_count`)
- When multiple keys hash to the same bucket (**collision**), we need a collision resolution strategy:
  - **Chaining**: Each bucket holds a linked list (or small array) of entries
  - **Open addressing**: Probe other buckets (linear probing, quadratic probing)
  - **Rust's `HashMap`**: Uses **SwissTable** (Google's design) with SIMD-optimized probing for speed

**Load factor & resizing:**

- Load factor = `num_entries / num_buckets`
- When load factor exceeds ~0.75, the hash map **resizes** (allocates a larger array, rehashes all entries) to maintain O(1) performance

**Time complexities:**

| Operation | Average | Worst case (many collisions) |
|-----------|---------|------------------------------|
| Insert    | O(1)    | O(n)                         |
| Lookup    | O(1)    | O(n)                         |
| Delete    | O(1)    | O(n)                         |

**Key terminology:**

1. **Entry** — a key-value pair
2. **Key** — unique identifier, used to generate the hash code
3. **Hash code** — integer computed from the key, determines the bucket index
4. **Bucket** — a slot in the array that stores one or more entries
5. **Value** — the data stored under the key

**Rust example:** See `./rust_specifics/src/data_collections/arrays_and_hashing/hash_structures/hashmaps/`

---

## 11. Multithreading

**How do you achieve thread-safe shared state? When would you use atomics vs a mutex vs channels?**

Use **synchronization primitives** to safely share state between threads or enable inter-thread communication.

**1. Atomics** (`AtomicBool`, `AtomicU32`, `AtomicUsize`, etc.)

- **What**: Lock-free operations on single scalar values using CPU atomic instructions (indivisible and uninterruptible)
- **When to use**:
  - Single variables (counters, flags, sequence numbers)
  - Very low overhead needed
  - High contention (many threads competing)
  - Lock-free algorithms (compare-and-swap loops)
- **Example**: `AtomicU64::fetch_add(1, Ordering::SeqCst)` for a shared counter

**2. Mutex** (`std::sync::Mutex<T>`, `tokio::sync::Mutex<T>`)

- **What**: Mutual exclusion lock — only one thread can access the protected data at a time. Uses atomic operations (test-and-set, compare-and-swap) internally to manage the lock
- **When to use**:
  - Complex types (structs, collections)
  - Multiple related fields that must be updated together
  - Low to moderate contention
  - Need to hold the lock across multiple operations
- **Variants**:
  - `std::sync::Mutex`: Blocks OS thread when waiting
  - `tokio::sync::Mutex`: Async-friendly, yields task when waiting
- **Example**: `Mutex<HashMap<String, User>>` for shared state

**3. RwLock** (`std::sync::RwLock<T>`)

- **What**: Multiple readers OR one writer
- **When to use**: Read-heavy workloads (many readers, infrequent writes)

**4. Channels** (`std::sync::mpsc`, `crossbeam::channel`, `tokio::sync::mpsc`)

- **What**: Message-passing queues that transfer ownership between threads
- **When to use**:
  - Task distribution (producer-consumer patterns)
  - Independent data streams (actor model)
  - Offloading unrelated work (parallelism)
  - Want to avoid shared mutable state ("Do not communicate by sharing memory; share memory by communicating")
- **Note**: Channels use **move semantics** (ownership transfer), not copy/clone by default. Some implementations are lock-free, others use locks internally.
- **Example**: `mpsc::channel()` to send work to a thread pool

**5. Arc** (`Arc<T>`)

- **What**: Atomic reference counting for shared ownership across threads
- **When to use**: Multiple threads need read access to the same data (combine with `Mutex` or `RwLock` for mutation)
- **Example**: `Arc<Mutex<T>>` for shared mutable state

---

## 12. Concurrency vs parallelism

**What is the difference between concurrency and parallelism?** Can you have one without the other?

**Concurrency** is about **task structure** — multiple tasks have overlapping lifetimes and can make progress independently, but not necessarily at the **same time**. They can be interleaved on a single core.

**Parallelism** is about **execution** — multiple tasks run at the **exact same time** on multiple CPU cores.

**Key distinction:**

- **Concurrency** = dealing with multiple things at once (structure)
- **Parallelism** = doing multiple things at once (execution)

**Can you have one without the other?** Yes.

**Examples:**

1. **Concurrent but not parallel**: Node.js or single-threaded Tokio — execution is managed by a scheduler and executor. Tasks are interleaved; only one runs at a time.
2. **Parallel but not concurrent**: Two processes on individual cores doing data analysis on the same data in different ways — splitting a struct into parts and running the same logic on different parts in parallel, with no interleaving. Or: SIMD (same operation on multiple data elements simultaneously).
3. **Both concurrent and parallel**: Tokio multi-threaded — spins up a per-core queue and scheduler with one global queue. Each core runs its own concurrent code taking items off the queue; cores can pull from the global queue if idle or work-steal from other queues.

**How does this relate to async/await?** Is async code concurrent? Is it necessarily parallel?

`async`/`await` relates heavily to **concurrent** programming — code that completes asynchronously doesn't return a value immediately like synchronous code. In Rust, async relates to **futures**: a future is something that will return a value at some point. It implements the `Future` trait with a `poll` function; once ready, that function returns a value. **Await points are yield points** — they indicate that this task is waiting for a future to become ready and will **yield** so other tasks can make progress.

**Is `await` blocking?** For the *task* — yes: the task is suspended and cannot continue until the future is ready. For the *thread* — no: the runtime yields the thread back to the executor, which can run other tasks on it. So: task blocked, thread not blocked. Contrast with `std::thread::sleep()` or blocking I/O, which would block the whole thread.

- **Is async code concurrent?** Yes — tasks can be interleaved.
- **Is it necessarily parallel?** No — depends on the runtime. Single-threaded Tokio = concurrent only; multi-threaded Tokio = both.

**What is a race condition?**

- A **race condition** occurs when multiple threads/tasks access shared mutable state without proper synchronization, and the outcome depends on the timing of execution
- **Parallelism** can cause races (multiple cores writing to same memory)
- **Concurrency** can also cause races (interleaved access on single core, e.g., signal handlers, re-entrant code)
- **Prevention**: Use atomics, mutexes, or message-passing (channels)

**When would you choose multiple threads (parallelism) vs a single thread with async (concurrency)?**

| Use multiple threads when:                          | Use single-threaded async when:                     |
|-----------------------------------------------------|-----------------------------------------------------|
| CPU-bound work (computation, data processing)       | I/O-bound work (network, disk, databases)           |
| Need true parallelism (utilize multiple cores)      | High concurrency with low CPU usage (10k+ connections) |
| Blocking operations that can't be made async        | Want low memory overhead (tasks are cheaper than threads) |
| Simple synchronous code is easier to reason about   | Need fine-grained task scheduling and cancellation  |

**Trade-offs:**

- **Threads**: Higher memory overhead (~2MB stack per thread), OS scheduling, easier to reason about (synchronous code)
- **Async**: Lower memory overhead (tasks are ~few KB), more complex (async/await, pinning, Send bounds), requires async-aware libraries

**How would you explain concurrency to someone who thinks it means “running at the same time”?**

Concurrency and parallelism are easy to confuse — they're related but different. **Concurrency** means two (or more) tasks have overlapping lifetimes but make progress at **different times**, not simultaneously. **Parallelism** means running at the **same time** on different OS threads and therefore CPUs (real or virtual cores) — it's more of an orchestration issue.

---

## 13. Time complexities (must know)

**What are the time complexities for:** array/Vec (unsorted and sorted), linked list, hash table, balanced BST/map, binary search, merge sort, quicksort (average)?

| Structure | Operation time complexity | Reason |
|-----------|---------------------------|--------|
| Array / Vec (unsorted) | O(1) access by index; O(n) search | Index is direct; search may need to scan every element. |
| Array / Vec (sorted) | O(1) access by index; O(log n) search | Binary search halves the search space each step. |
| Linked list | O(n) access by index, O(n) search | Must traverse from head (or tail) through nodes; no random access. |
| Hash table | O(1) average lookup/insert/delete; O(n) worst | Buckets give constant-time access; many collisions can degenerate to linear. |
| Balanced BST / map | O(log n) lookup/insert/delete | Tree height is O(log n) for n nodes; each step goes down one level. |
| Binary search | O(log n) | Halves the search space each iteration. |
| Merge sort | O(n log n) | Divide and conquer; always n log n, stable. |
| Quicksort (average) | O(n log n) average; O(n²) worst | Good pivot → split in half each time; bad pivot (e.g. sorted input) → degenerate. |

**General guide:**

- **O(log n)** — Running time grows slowly as n grows; the algorithm cuts the problem size by a constant factor (often half) each step.
- **O(n log n)** — Typical for efficient comparison sorts (merge, quicksort average).

**Example: O(n) vs O(log n) as n grows:**

| n | O(n) | O(log n) |
| --- | ------ | ---------- |
| 10 | 10 | ~3 |
| 100 | 100 | ~7 |
| 1,000 | 1,000 | ~10 |
| 1,000,000 | 1,000,000 | ~20 |

**Common Big O classes (fast → slow):**

| Complexity | Name | Description | How to recognise it |
|------------|------|-------------|----------------------|
| O(1) | Constant | Same work regardless of n | No loop over input; direct index, hash lookup, one comparison |
| O(log n) | Logarithmic | Work grows slowly; problem size shrinks by a factor each step | Loop/recursion halves (or similar) the problem each time; binary search, balanced tree ops |
| O(n) | Linear | Work proportional to n | Single loop over n elements; one pass through the data |
| O(n log n) | Linearithmic | Between linear and quadratic; typical for efficient sorts | Divide-and-conquer with linear merge per level; merge sort, heapsort, good quicksort |
| O(n²) | Quadratic | Work grows with square of n | Two nested loops over (same) n; bubble/insertion sort, naive two-sum |
| O(n^k) | Polynomial | k nested loops over n | k = 2 → quadratic, k = 3 → cubic, etc. |
| O(2^n) | Exponential | Work doubles (or similar) as n grows | Recursion/branching that tries many combinations; naive Fibonacci, subset enumeration |
| O(n!) | Factorial | Try every ordering of n items | Generate all permutations; brute-force TSP |

**How to decide what complexity an algorithm is:**

1. **Count loops over the input** — One loop of length n → often O(n). Two nested loops over n → often O(n²). Loop that halves n each time → O(log n).
2. **Check how the problem size shrinks** — Halves each step → O(log n). Shrinks by a constant (e.g. one element) each step → O(n).
3. **Worst vs average** — Use worst case unless the question says "average" (e.g. quicksort: O(n²) worst, O(n log n) average).
4. **Drop constants and lower-order terms** — 5n + 10 → O(n). 2n² + 3n → O(n²).

---

## 14. Rust-specific

**What are the main benefits of Rust?** (e.g. memory safety, no data races, zero-cost abstractions, performance.)

- **Memory safety without GC** — Ownership and borrowing enforce correct memory use at compile time (no dangling pointers, double free, or data races). No garbage collector: deterministic cleanup, no pauses, lower memory overhead.
- **Zero-cost abstractions** — High-level code (e.g. `Vec`, iterators) compiles to code as efficient as hand-written low-level code; you don't pay runtime cost for the abstraction.
- **Compile-time safety** — Monomorphization generates concrete code for each type used; the compiler catches many errors before run time.
- **Statically linked by default** — Binaries are self-contained (no runtime dependency on a Rust runtime); binaries can be larger but deployment is simple and predictable.
- **Fearless concurrency** — Ownership and borrowing plus `Send`/`Sync` let you write safe concurrent code; the compiler enforces thread safety.
- **Tooling and ecosystem** — Cargo, crates.io, and a strong focus on clear error messages and documentation.
- **Error handling** — `Result` and `Option` replace null/exceptions and force explicit handling; crates like thiserror and anyhow support structured errors.

**What is ownership?** What are the rules (single owner, move semantics, drop)? Why does Rust use it?

**What it is:** A set of compile-time rules that govern who “owns” each value and when memory is freed. Rust uses it to get memory safety without a garbage collector.

**Problems it prevents:** Dangling pointers, data races, double free, use-after-free, and other invalid memory access.

**Rules:**

1. **Single owner** — Each value has exactly one owner (the variable that holds it).
2. **Move semantics** — Assigning a value to another variable (or passing it to a function without a reference) transfers ownership; the original variable is no longer valid.
3. **Drop** — When the owner goes out of scope, the value is dropped and its memory is freed.

**What it enables:** Borrowing (references `&T` / `&mut T` for temporary access without taking ownership), lifetimes (compiler checks that references don’t outlive the data), and Copy/Clone for types that can be duplicated instead of moved.

**What is borrowing?** What is the difference between `&T` and `&mut T`? What are the borrowing rules (e.g. one mutable ref or many immutable refs, no dangling refs)?

**What it is:** Temporary access to a value without taking ownership, via references. Enforced by the **borrow checker** in the compiler.

**`&T` vs `&mut T`:**

- **`&T`** — Immutable reference: you can read but not modify. Many `&T` to the same value can exist at once.
- **`&mut T`** — Mutable reference: you can read and modify. No other reference (immutable or mutable) to that value can exist while the `&mut T` is in use.

**Rules:**

1. **Either many immutable refs OR one mutable ref** — Not both at the same time. (You wrote “many mutable” by mistake; the rule is many **immutable** or one **mutable**.)
2. **References must be valid** — A reference must not outlive the data it points to (lifetimes enforce this).
3. **No overlapping mutable access** — While data is borrowed mutably, nothing else can read or write it; this avoids data races in single-threaded code (Send/Sync handle threads).

**What is monomorphization?** How does Rust use it for generics, and what are the trade-offs (e.g. compile time, binary size, runtime cost)?

**What it is:** The compiler generates a separate copy of each generic function or type for every concrete type it’s used with. One generic implementation is “morphed” into many concrete ones at compile time.

**Trade-offs:**

- **Runtime:** No extra cost — no vtables, inlining works; same performance as hand-written code per type.
- **Binary size:** Can grow — one copy per concrete type.
- **Compile time:** Can increase — more code to generate and optimise.

**When would you use generics (`fn f<T: Trait>(x: T)`) vs trait objects (`dyn Trait`)?** What is the runtime cost of each?

**Generics (static dispatch):**

- **When:** You know the concrete type at compile time and want maximum performance.
- **How:** Monomorphization — one implementation per type; calls are inlined or direct.
- **Cost:** Zero extra runtime cost; larger binary and longer compile time.

**Trait objects (dynamic dispatch, `dyn Trait`):**

- **When:** You need a heterogeneous collection or don’t know the concrete type until runtime (e.g. plugin-style code).
- **How:** Fat pointer (data pointer + vtable); each call goes through the vtable.
- **Cost:** Small runtime cost (vtable lookup, no inlining); smaller binary and more flexibility at runtime.

**Rule of thumb:** Prefer generics for performance-critical paths; use `dyn Trait` when you need runtime polymorphism or to reduce binary size.

**What do `Send` and `Sync` mean?** When is a type `Send`? When is it `Sync`? How does Rust use them for thread safety?

**What they are:** Marker traits (no methods) that describe whether a type is safe to use across threads. The compiler uses them to enforce thread safety at compile time.

**Send** — Safe to **transfer ownership** to another thread (e.g. pass a value across a channel). The type can be moved to another thread.

**Sync** — Safe to **share** between threads via references. If `T: Sync`, then `&T` can be sent to another thread (multiple threads can hold `&T` concurrently). Not “synchronised” in the lock sense — it means “safe to share by reference.”

**Rule of thumb:** `T: Send` ⇒ “I can give this to another thread.” `T: Sync` ⇒ “I can share a reference to this with another thread.”

Most primitives and many standard types are `Send + Sync`; the compiler infers or requires these bounds when you use threads, `Arc`, channels, etc.

**When would you use `Arc` vs `Rc`?** What about `Mutex<T>` vs `RwLock<T>` for shared mutable state?

**Arc vs Rc:**

- **Rc** — Reference count; non-atomic. Single-threaded only; **not** `Send` or `Sync`. Use when multiple parts of the same thread need to share ownership of the same value.
- **Arc** — **A**tomic **R**eference **C**ount; the count is updated with atomic operations. **Send + Sync**. Use when multiple threads need to share ownership of the same value (e.g. `Arc<Mutex<T>>` for shared mutable state across threads).

**Mutex vs RwLock:**

- **Mutex** — Mutual exclusion: only one holder at a time, for either reading or writing. Simple and predictable. Use when you need exclusive access or when contention is low.
- **RwLock** — Multiple readers OR one writer. Use when reads are common and writes are rare (read-heavy workload). More overhead than Mutex; can be slower than Mutex if writes are frequent.

**What is the purpose of `Result` and `Option`?** How do they replace null/exceptions and enforce error handling?

**Option&lt;T&gt;** — Represents “maybe no value”: either `Some(T)` or `None`. Replaces null/undefined; the type system forces you to handle the “no value” case (e.g. `match`, `unwrap_or`, `?`).

**Result&lt;T, E&gt;** — Represents “value or error”: either `Ok(T)` or `Err(E)`. Replaces unchecked exceptions; the type system forces you to handle or propagate errors (e.g. `match`, `?`, `unwrap`/`expect` for tests or when you’re sure).

**Why Rust uses them:** No null or undefined behaviour; errors are explicit and must be handled or propagated. Idiomatic error handling: use `?` to propagate, `thiserror`/`anyhow` for structured errors.

**What does “zero-cost abstraction” mean in Rust?** Can you give an example?

**What it means:** If you wrote the ideal low-level code by hand, you wouldn't do better. The abstraction doesn't add runtime cost — no extra indirection, no hidden allocations; the compiler inlines and optimises so that the high-level code is as fast as hand-written code.

**Example — Vec:** Using `Vec<T>` is as efficient as manually managing a pointer, capacity, and length. Iterators compile to loops with no extra cost. Under the hood, `Vec` uses something like `RawVec` (lazy allocation on first `push`, capacity growth, cleanup on drop) — you don't pay more than you would if you wrote it yourself. Same idea for `Option`/`Result`: they compile to the same representation as a nullable pointer or tagged union; no runtime penalty for the abstraction.

---

## 15. Extra topics (from job description)

- **Web:** When would you use REST vs WebSockets?

**REST (request–response):**

- **Model:** Client sends a request; server sends one response. Stateless; each request is independent.
- **Use when:** You need a one-off answer to a question (queries, CRUD, “give me this resource”). Good for APIs where the client initiates and waits for a single response.
- **Example:** “Get order book snapshot,” “place order,” “fetch user profile.”

**WebSockets (full-duplex, persistent connection):**

- **Model:** One long-lived connection; both sides can send messages at any time. Full-duplex = client and server can push data independently.
- **Use when:** You need **streaming** or **real-time updates** (market data ticks, live order book, notifications, chat). Avoids repeatedly opening connections and reduces latency once the connection is up.
- **Example:** Live price feed, order book updates, trade notifications, dashboards that update in real time.

**Rule of thumb:** REST for “ask once, get one answer.” WebSockets for “keep a connection open and stream or push events.”

---

## 16. High-frequency / low-latency systems (systems view)

### **Memory & allocation**

#### Why avoid heap allocation in the hot path? What is the cost of malloc/free or of growing a `Vec`?

**Hot path** — The latency-sensitive path (e.g. handling every order or tick). We want predictable, low latency there; allocation adds variable, unpredictable cost.

**Why heap is worse than stack:** Heap is for data whose size or lifetime isn’t known at compile time (e.g. `String`, `Vec`). Allocation goes through the global allocator (often involving OS and bookkeeping). Cost is **unpredictable**: sometimes fast, sometimes slow when the allocator extends the heap or coalesces blocks. In multi-threaded code the allocator can also be a **contention** point.

**Cost of growing a `Vec`:** When `len == cap`, the next `push` triggers growth: (1) allocate a new buffer (typically 2× capacity), (2) copy/move all elements, (3) free the old buffer. So you get a one-off **spike** (allocation + copy + free) at an **unpredictable** time. As the Vec grows, each reallocation copies more data (5 → 10 → 20 → 40 …). Downsides: **OOM** in memory-constrained systems, **fragmentation** over time, and **latency spikes** on the hot path.

**What to do instead:** Pre-allocate (e.g. `Vec::with_capacity`), use the stack where possible, or reuse buffers (object pools, arena allocators) so the hot path doesn’t call malloc/free or trigger Vec growth.

#### When would you use object pools, arena allocators, or pre-allocated buffers instead of allocating on demand?

**When:** In the **hot path** or in latency-sensitive code where you want to avoid malloc/free and unpredictable allocation cost.

- **Object pool** — Pre-allocate a fixed set of objects (e.g. order structs, messages). When you need one, take it from the pool; when done, return it. No malloc/free in the hot path; you reuse the same memory. Use when you have a bounded number of short-lived objects of the same type (e.g. order objects in a matching engine).

- **Arena allocator** — One big block of memory; you hand out slices from it. Allocations are just pointer bumps (very fast). You free the whole arena at once (e.g. at end of request or end of frame), so no per-object free. Use when you have many allocations that all die at the same time (e.g. parsing a message, building a response).

- **Pre-allocated buffer** — Allocate once (e.g. `Vec::with_capacity(n)` or a fixed-size array on the stack) and reuse. No growth, no reallocation in the hot path. Use when you know an upper bound on size (e.g. max order book depth, max message size).

**Rule of thumb:** If the hot path currently calls malloc/free or grows a Vec, consider pools (reuse objects), arenas (batch free), or pre-allocated buffers (fixed size) so the hot path doesn’t touch the global allocator.

#### What is cache-friendly data layout? Why does it matter for latency?

**What it is:** Organising data in memory so the CPU uses its **cache** efficiently. When the CPU reads a memory address, it loads a **cache line** (e.g. 64 bytes) into L1/L2/L3 cache. If the next access is in that same line, it’s fast (cache hit); if not, it’s a miss (slow, fetch from RAM).

**Cache-friendly layout:**

- **Sequential access** — Process data in the order it sits in memory (e.g. iterate a `Vec` front to back). The CPU prefetches the next cache lines; you get mostly hits.
- **Contiguous data** — Keep related data together (e.g. array of structs `[Order; N]` so all orders are in a row). One cache line can hold several orders; one load serves multiple uses.
- **Avoid random jumps** — If you follow pointers all over the heap (e.g. linked list, tree with pointers), each node may be in a different cache line → many misses.

**Why it matters for latency:** Cache hit is on the order of **nanoseconds**; cache miss can be **hundreds of nanoseconds** (or more if you go to main memory). In HFT, the hot path is measured in nanoseconds, so a few extra cache misses can dominate. Cache-friendly layout keeps the hot path in cache and reduces tail latency.

#### What is false sharing? How can padding or per-core data reduce it?

**What it is:** **False sharing** is when two different variables used by two different CPU cores end up in the **same cache line**. When Core A writes to its variable, the cache line is invalidated; Core B’s copy of that line is invalidated too, so when Core B reads its variable it gets a **cache miss** and has to fetch the line again. They’re not actually sharing the same variable — they’re just sharing a cache line — so you get unnecessary contention and slowdown.

**Example:** Two counters, one per thread, placed next to each other in memory. Thread 1 increments `counter_a`, thread 2 increments `counter_b`. If both are in the same 64-byte cache line, each increment causes the other thread to miss on its next read.

**How to reduce it:**

- **Padding** — Put padding (or align) so each frequently written variable sits in its **own** cache line. E.g. `struct Counter { value: AtomicU64; _pad: [u8; 56]; }` so the next counter starts in a new line. Then each core’s writes don’t invalidate the other’s line.
- **Per-core data** — Give each core its own copy of the data (e.g. one counter per core, or per-thread buffers). No sharing of cache lines between cores, so no false sharing. You combine results only when needed (e.g. at the end of a phase).
- **Cache-line alignment** — Use `#[repr(align(64))]` (or similar) so a struct starts at the beginning of a cache line; then padding to the next line for the next instance.

**Why it matters for latency:** In low-latency code, multiple threads often update different fields (e.g. per-core stats). If those fields share a cache line, you get invisible contention and variable latency. Padding or per-core data removes that.

### **CPU & cache**

A cache is a high speed temporary data storage layer that stores frequently accessed info in the aim of allowing it to be retrieved faster. This improves performance by reducing data fetching latency allowing data to be processed faster.

A CPU is a Central Processing Unit — it is the brain of the computer, interpreting and executing instructions.

**Hardware cache hierarchy — typical access times (order-of-magnitude):**

| Level | Name | Typical size (per core or shared) | Access time (approx.) | Notes |
| ------- | ------ | ----------------------------------- | ------------------------ | -------- |
| L1 cache | L1 data / L1 instruction | 32–64 KB (per core) | **~1 ns** | Fastest; per core; often split I/D. |
| L2 cache | L2 | 256 KB – 1 MB (per core) | **~3–4 ns** | Per core on many CPUs. |
| L3 cache | L3 (LLC) | 8–64 MB (shared across cores) | **~12–40 ns** | Last-level cache; shared; larger but slower. |
| Main memory | RAM | GBs | **~60–100 ns** | Cache miss to DRAM. |
| SSD | NVMe / SATA SSD | GBs–TBs | **~50–150 μs** | I/O; not a CPU cache. |
| HDD | Disk | TBs | **~1–10 ms** | I/O; much slower. |

*Exact numbers vary by CPU (Intel, AMD, ARM) and generation. In HFT, L1/L2 hits are desirable in the hot path; L3 and especially RAM misses add significant, variable latency.

#### What is a cache line? How can contention on a single cache line hurt performance across cores?

**What is a cache line?**  
A cache line is the **unit of data** the CPU uses for caching — typically **64 bytes**. When you read or write an address, the CPU loads (or keeps coherent) the **entire line** containing that address. So one access brings in 64 bytes; any later access to bytes in that same line can be served from cache (fast). It’s not “one type” or a queue — it’s just the fixed-size block the hardware uses to move data between RAM and cache (and between cache levels).

**How can contention on a single cache line hurt performance across cores?**  

The cache line is also the **unit of coherence**: when one core writes to any byte in a line, other cores’ copies of that **entire line** are invalidated (e.g. MESI). So:

- **False sharing** — Two cores write to **different variables** that happen to lie in the **same** 64-byte line. Each write invalidates the other’s copy of the line → the line “ping-pongs” between caches → extra cache misses and coherence traffic. No mutex needed; bad layout alone causes the hurt.
- **True sharing** — Two cores access the **same** variable (e.g. a counter). You need synchronisation (mutex/atomic), and the shared line is hot. Every write invalidates other cores’ copies → they miss and re-fetch the line → more latency and contention.

In both cases, performance suffers from **cache misses** and **coherence traffic**, not only from waiting on a lock.

#### Why might you pin threads to specific CPU cores (CPU affinity)? What problem does it solve?

You ideally want to pin threads to cores when the work is:

1. Highly CPU intensive work
2. Not await heavy, or network heavy where you are waiting - this will leave the core unused for a period of time.

#### Why do predictable branches and straight-line code paths help in low-latency code?


### **Kernel & syscalls**

- What is the cost of a syscall (context switch, kernel/user boundary)? When should you avoid syscalls in the hot path?


- What is kernel bypass (e.g. DPDK, io_uring)? When would you use it instead of the normal OS network stack?
- When might you prefer busy-wait / polling over blocking (e.g. interrupts) in a latency-critical loop?

### **Concurrency & locking**

- When do lock-free structures (atomics, lock-free queues) beat mutexes in low-latency systems? When might a mutex still be better?

Lock free 
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

**Prep:** Fill in the trading bot diagram and risks at each stage → [trading-bot-risk-prep.md](./trading-bot-risk-prep.md)

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

Garbage Collectors - what are they why doesn't rust need them?
Stack vs Heap

When do you allocate memory to them both, how do you do it?