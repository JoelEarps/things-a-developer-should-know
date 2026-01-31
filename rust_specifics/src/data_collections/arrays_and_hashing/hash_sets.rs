/*
Hash sets are a collection of unique values, they are very similar to hashmaps but they only store keys and not values
It is a form of a hash table, typically used to see if a value exists in the set or not
A hash set stores the unique elements in buckets according to the elements hash code.

The value wishing to be added to the set is hashed to produc a bucket index

The hashset is often implemented as an array of buckets.
When multiple values hash to the same bucket, rust uses chaining (each bucket stores an array/ vector/ linked list of vals).
How can this happen? well the hash code is too big to use as a bucket index so we use an operation such as modulo (the standard) to

Time complexity:
1. Insert - average O(1), worst case O(n)
2. Search - average O(1), worst case O(n)
3. Delete - average O(1), worst case O(n)


Hashset vs vec:
Use hashset when you want fast look ups, uniqueness/ set operations like union (combine sets and remove duplicates)
Use vec when - order matters, you want index access, duplicates are allowed or you want small set sizes
*/

// Implementing my own hashset - what functions do we need?

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_hashset_insert() {
        let mut hashset = HashSet::new();
        hashset.insert(1);
        hashset.insert(2);
        hashset.insert(1);

        println!("{:?}", hashset.len()); // Should be 2 as 1 is duplicate

        // Fun things and combinators you can use
    }
}
