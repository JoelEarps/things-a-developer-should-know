## `std::collections::HashMap` — key ideas and how it compares to `MyHashMap`

### What `std::collections::HashMap` gives you

- **Buckets + probing**: Stores entries in an internal array of buckets and uses a probing strategy
  (Robin Hood hashing in modern Rust) to find slots with good cache locality.
- **Randomized hashing by default**: Uses `RandomState` (SipHash-based, or AHash in some builds)
  to make hash DoS attacks harder by randomizing the mapping from keys to buckets.
- **Load factor + resizing**: Automatically resizes when the table gets too full so that
  operations stay \(O(1)\) on average.
- **Rich API**:
  - `insert`, `get`, `get_mut`, `remove`, `entry` (with `or_insert`, `or_default`, etc.).
  - Iterators over keys, values, and key/value pairs.
- **Deterministic complexity guarantees**: Amortized \(O(1)\) for insert / lookup / remove under
  reasonable hash behavior.

### How our `MyHashMap` is structured

File: `custom_hashmap.rs`

- **Buckets**: `Vec<Vec<(K, V)>>` — a vector of buckets, each bucket is a `Vec` of `(K, V)` pairs.
- **Collision handling**: **separate chaining** — when two keys hash to the same bucket index,
  both `(K, V)` pairs live in the same small vector and we linearly scan that vector.
- **Hashing**: Uses a generic `BuildHasher` (defaults to `RandomState`) so it can behave
  similarly to `std::collections::HashMap`, but keeps the code simple.
- **Growth**:
  - Starts with a small number of buckets.
  - When `len / buckets.len()` exceeds 0.75, we allocate a new bucket array and re-hash all keys.
- **Core API**:
  - `new`, `with_hasher`
  - `len`, `is_empty`
  - `insert`, `get`, `get_mut`, `remove`

### Side-by-side comparison

#### Collision strategy

- **`MyHashMap`**:
  - Separate chaining: collisions become a small vector search.
  - Simpler to reason about and implement.
  - Easy to visualize buckets: each index has a `Vec` of entries.
- **`std::collections::HashMap`** (current implementation, Robin Hood hashing):
  - Open addressing with probing (no nested vectors inside buckets).
  - Tries to keep probe lengths small and even by “stealing” good positions from entries with
    shorter probe distance.
  - Better cache locality than chaining for many workloads.

#### Memory layout

- **`MyHashMap`**:
  - Buckets: `Vec<Vec<(K, V)>>` → an outer `Vec` of inner `Vec`s.
  - Each `(K, V)` pair is stored inside an inner `Vec`, which may allocate separately.
  - Simpler, but more pointer chasing and potential heap fragmentation.

- **`std::collections::HashMap`**:
  - Stores control bytes and key/value pairs in contiguous arrays.
  - Carefully tuned to reduce cache misses when searching for keys.
  - More complex internals, but faster in practice for real workloads.

#### Hashing and security

- **`MyHashMap`**:
  - Uses whatever `BuildHasher` you provide (defaults to `RandomState`).
  - You can also plug in a deliberately “bad” hasher in tests to force collisions and
    understand behavior.

- **`std::collections::HashMap`**:
  - Defaults to `RandomState`, which seeds the hasher with randomness.
  - Makes it hard for an attacker to craft many different keys that all land in the same bucket.
  - You can choose a different hasher (e.g. `FxHash`) if you want speed over attack resistance.

#### API surface

- **`MyHashMap`** (in this repo):
  - Minimal: `insert`, `get`, `get_mut`, `remove`, `len`, `is_empty`.
  - Great for **learning the core ideas**: buckets, hashing, collisions, resizing.

- **`std::collections::HashMap`**:
  - Much larger API, including:
    - `entry` API for in-place initialization and mutation.
    - `retain`, `drain`, `keys`, `values`, `values_mut`, `into_iter`, etc.
  - Tuned for ergonomics and flexibility in real applications.

### When to use which

- **Use `MyHashMap`** when:
  - You want to understand how hash maps work internally.
  - You want a small, readable implementation to step through with a debugger.
  - You’re experimenting with alternative collision strategies or hashers.

- **Use `std::collections::HashMap`** when:
  - Writing production code.
  - You need performance, safety, battle-tested behavior, and a full API.

