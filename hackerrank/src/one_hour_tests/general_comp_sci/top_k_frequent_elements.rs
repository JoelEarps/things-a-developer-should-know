/*
Question:
Problem: Top-K Frequent Elements

Prompt:
Given an array of integers, return the k most frequent elements.

Requirements:
Input: Array of integers (nums), integer k
Output: Array/Vector of the k most frequent elements

Constraints:
- Large input size
- Order does not matter (can return in any order)
- k is in the range [1, number of unique elements in the array]
- It is guaranteed that the answer is unique

What this tests:

Hash maps (for frequency counting)
Heaps (priority queues for top-k)
Frequency counting
Algorithm optimization
Trade-offs between approaches

Follow-ups (very common):
- Can you do better than sorting?
- What if the stream is infinite?
- What if k is very large (close to array size)?
- What if we need to handle ties?

Clarify:

TODO: Add your clarifying questions here
1. What is the format of the input?
2. What should we return if there are ties (same frequency)?
3. Can k be larger than the number of unique elements?
4. What if k is 0 or negative?
5. What if the array is empty?

## Handling Ambiguity

Best way to handle ambiguity is to ask the interviewer for clarification. However lets say the interviewer is not clear and says something like
"Do what you want? Or make your own decisions". Then you need an approach:
1. Make reasonable assumptions and document them
2. Show reasoning
3. Be flexible - you can always change your approach later if needed as information and assumptions are clarified

The key thing here is use your own experience - if not be sensible!

Top-K Frequent Elements ambiguity analysis:

TODO: Add your assumptions here

Example walkthrough:

TODO: Add your example here

Paraphrase - a collection of data is passed in and we need to find the elements that appears most often with the total returned amounting to k
So
Example - lets say we have the following array: [1, 1, 1, 2, 2, 3] and k = 2
We want to return the 2 most frequent elements which are  1 and 2
Ambiquity

1. What happens if there are ties?

What i will assume here is that we could do one of two things:
1. Return all the largest in the tie
2. Return the one that appears first e.g. an LRU cache
3. Let fate decide e.g. access from the compiler and runtime

For now lets assume this will not happen

2. Can k be larger than the number of unique elements
3. Are the elements all unsigned - lets assume no
4. Max value of the elements - lets as i32

Failure states and early returns
k is larger than the number of unique elements
k is 0 or negative
array is empty
k is smaller than the number of frequent elements i.e. k is 2 and there are 3 unique elements that all appear once




Plan

Brute force - iterate though the array and get the next element
If it has not occurred add it to a hashmap
If it has has occured increment the count

Loop through the hashmap and get the top k elements

Print


Implement - lets a go

Evaluate

1. For loop O(log(n)) -
O(n + m log m) → worst-case O(n log n)

Bottle neck is for loop - what you could do to speed up is the following:
1. Sliding window pattern - batch process
2. Add some parallelism and concurrency primitives if data is super huge - rayon etc, break into chunks and process
Will require synchronization prims - but I would use Arc Mutex, or maybe some sort of Arc atomics (not sure if the key would be safe as writing might be an issue)

Space complexity
What is the space complexity of a vecque - O(k) as length is proportional to number of elements - with capacity - ver element i32 is 4 bytes, so it is 4*k bytes
What is the space complexity of a hashmap - O(n) as we have to store all keys and values as a hashtable and and size of which is proportional to the number of vals and keys

So total space is O(n + k)

*/

use std::collections::{HashMap, VecDeque};

/// Finds the k most frequent elements in the array
///
/// # Arguments
/// * `nums` - Array of integers
/// * `k` - Number of most frequent elements to return
///
/// # Returns
/// Vector containing the k most frequent elements (in any order)
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // Create a hashmap
    let mut count_map: HashMap<i32, usize> = HashMap::new();

    // for each value iterate through and insert or update the current value
    for num in nums {
        let entry = count_map.entry(num).or_insert(0);
        *entry += 1;
    }

    // We can use a queue to push/ pop elements off the front of the queue that are smaller than the last element in the queue
    // This way we can save sorting
    // let mut counting_queue: VecDeque<i32> = VecDeque::with_capacity(k as usize);
    // // Have a value lowest count that can be used to store the value at the back of the queues count so it can be popped off if one surpasses it
    // // This way the queue will be stored in decsending order which is nice
    // let mut lowest_count = 0;
    // // for (key, value) in count_map {
    // //     // If the queue is smaller than k we just append
    // //     if counting_queue.len() < k as usize {
    // //         counting_queue.push_front(key);
    // //         lowest_count = value;
    // //     } else {
    // //         // if the current count is higher than the lowest count stored
    // //         // push new key onto the front of the queue and move the valye off the end
    // //         if lowest_count < value {
    // //             counting_queue.pop_back();
    // //             counting_queue.push_front(key);
    // //             lowest_count =
    // //         } else {
    // //             println!("Lowest count in queue {} is more than that the one currently being searched : {}", lowest_count, value);
    // //         }
    // //     }
    // // }

    
    // Convert to vector of (element, count) pairs and sort
    let mut pairs: Vec<(i32, usize)> = count_map.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending

    pairs
        .into_iter()
        .take(k as usize)
        .map(|(element, _)| element)
        .collect()
}

#[cfg(test)]
mod top_k_frequent_elements_tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_basic() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = top_k_frequent(nums, k);
        // Expected: [1, 2] (or [2, 1] - order doesn't matter)
        // 1 appears 3 times, 2 appears 2 times, 3 appears 1 time
        // Top 2: 1 and 2
        assert_eq!(result.len(), k as usize);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[test]
    fn test_top_k_frequent_single_element() {
        let nums = vec![1];
        let k = 1;
        let result = top_k_frequent(nums, k);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_top_k_frequent_all_same() {
        let nums = vec![1, 1, 1, 1];
        let k = 1;
        let result = top_k_frequent(nums, k);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_top_k_frequent_k_equals_unique() {
        let nums = vec![1, 1, 2, 2, 3, 3];
        let k = 3;
        let result = top_k_frequent(nums, k);
        // All three elements have the same frequency (2 each)
        assert_eq!(result.len(), k as usize);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_top_k_frequent_larger_example() {
        let nums = vec![4, 1, -1, 2, -1, 2, 3];
        let k = 2;
        let result = top_k_frequent(nums, k);
        // Frequencies: -1: 2, 2: 2, 1: 1, 3: 1, 4: 1
        // Top 2: -1 and 2 (both appear 2 times)
        assert_eq!(result.len(), k as usize);
        assert!(result.contains(&-1));
        assert!(result.contains(&2));
    }
}
