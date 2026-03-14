//! # Custom HashMap — learning-focused implementation
//!
//! A minimal hash map built from scratch to understand buckets, hashing, collision handling,
//! and resizing. Not a replacement for `std::collections::HashMap` in production.
//!
//! ## Components overview
//!
//! ### 1. **Buckets** (`Vec<Vec<(K, V)>>`)
//! - The map is an array of **buckets**. Each bucket is a small `Vec` of `(key, value)` pairs.
//! - A key’s **hash** (via the hasher) is turned into a **bucket index** with `hash % num_buckets`.
//! - Different keys can land in the same bucket (**collision**); we store all of them in that
//!   bucket’s list and scan it when looking up or removing.
//!
//! ### 2. **Separate chaining**
//! - Collisions are handled by **chaining**: each bucket holds a list (here, a `Vec`) of entries.
//! - Lookup: hash key → bucket index → linear scan of that bucket for a matching key.
//! - Alternative (used by `std`): open addressing (e.g. Robin Hood), where entries live in a
//!   single flat array and we “probe” to the next free slot. Chaining is simpler to implement.
//!
//! ### 3. **Hasher** (`S: BuildHasher`)
//! - The generic `S` builds a **Hasher** for each hash we need. We feed the key into the hasher,
//!   then call `finish()` to get a `u64`. That value is reduced to a bucket index with `% len`.
//! - Default is `RandomState` (SipHash-style), which is what `std::collections::HashMap` uses
//!   by default — good for DoS resistance. You can plug in a custom hasher (e.g. for tests that
//!   force collisions).
//!
//! ### 4. **Load factor and resizing**
//! - **Load factor** = `len / num_buckets`. When it exceeds a threshold (here 0.75), we **resize**:
//!   allocate a new, larger bucket array (e.g. 2× the size), then **rehash** every key into the
//!   new buckets. This keeps buckets short and lookups fast.
//!
//! ### 5. **Length** (`len: usize`)
//! - Total number of key-value pairs. We maintain it so `len()` is O(1) and we can compute load
//!   factor without iterating.
//!
//! ## Operation summary
//!
//! | Operation | What happens |
//! |-----------|----------------|
//! | `insert(k, v)` | Maybe resize; hash `k` → bucket; if `k` already in that bucket, replace value and return old; else push `(k,v)` and increment `len`. |
//! | `get(k)`       | Hash `k` → bucket; scan bucket for key equal to `k`; return `Some(&v)` or `None`. |
//! | `get_mut(k)`   | Same as `get` but return `Some(&mut v)`. |
//! | `remove(k)`    | Hash `k` → bucket; find position of `k` in bucket; swap_remove that entry; decrement `len`; return the value. |

use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

/// Number of buckets when the map is first created. Kept small so resizing is easy to observe.
const DEFAULT_BUCKETS: usize = 16;
/// When (len / num_buckets) exceeds this, we double the number of buckets and rehash.
const MAX_LOAD_FACTOR: f64 = 0.75;

/// Learning-focused hash map: separate chaining, configurable hasher, resize on load factor.
#[derive(Debug)]
pub struct MyHashMap<K, V, S = RandomState> {
    /// One bucket per index; each bucket is a list of (key, value) pairs (collision chain).
    buckets: Vec<Vec<(K, V)>>,
    /// Total number of key-value pairs (so we can compute load factor and expose O(1) len()).
    len: usize,
    /// Builds a hasher for each hash we need (key → u64 → bucket index).
    hasher: S,
}

impl<K, V> MyHashMap<K, V> {
    /// Create a new map with default capacity and the standard library’s default hasher
    /// (`RandomState`), which is good for DoS-resistant hashing.
    pub fn new() -> Self {
        Self::with_hasher(RandomState::new())
    }
}

impl<K, V, S> MyHashMap<K, V, S> {
    /// Create a new map with the given hasher. Use this when you want a custom hasher
    /// (e.g. a deterministic or “bad” hasher for tests that force collisions).
    pub fn with_hasher(hasher: S) -> Self {
        // Allocate the bucket array: one empty Vec per bucket.
        let mut buckets = Vec::with_capacity(DEFAULT_BUCKETS);
        buckets.resize_with(DEFAULT_BUCKETS, Vec::new);

        Self {
            buckets,
            len: 0,
            hasher,
        }
    }

    /// Number of key-value pairs in the map. O(1).
    pub fn len(&self) -> usize {
        self.len
    }

    /// True if there are no entries. O(1).
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K, V, S> MyHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    /// Compute the bucket index for a key: hash the key with our hasher, then reduce modulo
    /// bucket count so the index is in `0..buckets.len()`. `Q` allows lookup by a type that
    /// differs from `K` as long as `K: Borrow<Q>` (e.g. look up `String` by `&str`).
    fn bucket_index_for<Q>(&self, key: &Q) -> usize
    where
        Q: ?Sized + Hash,
    {
        let mut state = self.hasher.build_hasher();
        key.hash(&mut state);
        let hash = state.finish();
        (hash as usize) % self.buckets.len()
    }

    /// Load factor = number of entries / number of buckets. High load factor means long chains
    /// and slower lookups; we resize when this exceeds `MAX_LOAD_FACTOR`.
    fn load_factor(&self) -> f64 {
        if self.buckets.is_empty() {
            return 0.0;
        }
        self.len as f64 / self.buckets.len() as f64
    }

    /// If the load factor is above the threshold, double the number of buckets and rehash every
    /// entry into the new bucket array. Entries may move to different buckets because
    /// `index = hash % num_buckets` and `num_buckets` has changed.
    fn try_resize(&mut self) {
        if self.load_factor() <= MAX_LOAD_FACTOR {
            return;
        }

        let new_bucket_count = self.buckets.len().saturating_mul(2).max(1);
        let mut new_buckets = Vec::with_capacity(new_bucket_count);
        new_buckets.resize_with(new_bucket_count, Vec::new);

        // Drain each old bucket and re-insert every (k, v) into the new bucket array.
        for bucket in self.buckets.drain(..) {
            for (k, v) in bucket {
                let mut state = self.hasher.build_hasher();
                k.hash(&mut state);
                let hash = state.finish();
                let idx = (hash as usize) % new_bucket_count;
                new_buckets[idx].push((k, v));
            }
        }

        self.buckets = new_buckets;
    }

    /// Insert a key-value pair. If the key already exists, the value is replaced and the previous
    /// value is returned. Otherwise returns `None`. May trigger a resize before inserting.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.try_resize();
        let idx = self.bucket_index_for(&key);
        let bucket = &mut self.buckets[idx];

        // Scan the chain for an existing entry with the same key; if found, replace and return old value.
        for (existing_key, existing_value) in bucket.iter_mut() {
            if existing_key == &key {
                return Some(std::mem::replace(existing_value, value));
            }
        }

        // Key not found: append to the chain and increment total count.
        bucket.push((key, value));
        self.len += 1;
        None
    }

    /// Look up a key by reference. Uses `Borrow<Q>` so you can e.g. look up by `&str` when
    /// keys are `String`. Returns a reference to the value if present.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        let idx = self.bucket_index_for(key);
        self.buckets[idx]
            .iter()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    /// Mutable lookup: same as `get` but returns `Option<&mut V>` so the value can be updated in place.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        let idx = self.bucket_index_for(key);
        self.buckets[idx]
            .iter_mut()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    /// Remove the entry for the given key and return the value if it was present. Uses
    /// `swap_remove` so the removal is O(1) at the cost of reordering the chain.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        let idx = self.bucket_index_for(key);
        let bucket = &mut self.buckets[idx];

        let pos = bucket.iter().position(|(k, _)| k.borrow() == key)?;
        let (_, v) = bucket.swap_remove(pos);
        self.len -= 1;
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::MyHashMap;

    /// Basic insert and get; keys that hash to different buckets.
    #[test]
    fn insert_and_get() {
        let mut map = MyHashMap::new();
        assert!(map.is_empty());

        map.insert("one", 1);
        map.insert("two", 2);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&"one"), Some(&1));
        assert_eq!(map.get(&"two"), Some(&2));
        assert_eq!(map.get(&"three"), None);
    }

    /// Insert same key twice; second insert replaces value and returns the old one.
    #[test]
    fn update_existing_key() {
        let mut map = MyHashMap::new();
        assert_eq!(map.insert("key", 1), None);
        assert_eq!(map.insert("key", 2), Some(1));
        assert_eq!(map.get(&"key"), Some(&2));
        assert_eq!(map.len(), 1);
    }

    /// Remove an existing key; map is empty afterwards and get returns None.
    #[test]
    fn remove_key() {
        let mut map = MyHashMap::new();
        map.insert("key", 10);
        assert_eq!(map.remove(&"key"), Some(10));
        assert_eq!(map.get(&"key"), None);
        assert!(map.is_empty());
    }

    /// Uses a hasher that always returns 0 so all keys land in the same bucket; verifies
    /// that chaining still yields correct get/len.
    #[test]
    fn handles_collisions() {
        use std::hash::{BuildHasher, Hasher};

        #[derive(Clone)]
        struct BadHasherBuilder;

        struct BadHasher(u64);

        impl Hasher for BadHasher {
            fn write(&mut self, bytes: &[u8]) {
                for b in bytes {
                    self.0 = self.0.wrapping_add(*b as u64);
                }
            }

            fn finish(&self) -> u64 {
                0
            }
        }

        impl BuildHasher for BadHasherBuilder {
            type Hasher = BadHasher;

            fn build_hasher(&self) -> Self::Hasher {
                BadHasher(0)
            }
        }

        let mut map = MyHashMap::<&str, i32, BadHasherBuilder>::with_hasher(BadHasherBuilder);
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), Some(&3));
        assert_eq!(map.len(), 3);
    }
}

