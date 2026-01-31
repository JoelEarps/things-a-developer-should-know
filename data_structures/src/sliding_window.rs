/*
Sliding window is a technique for processing sequences by maintaining a fixed-size window that slides through the sequence.
You keep a sub array and then slide it one fixed step at a time

Array: [1, 3, 2, 4, 5, 6, 7]
       Window size: 3

Step 1: [1, 3, 2] ← Window here
Step 2:    [3, 2, 4] ← Slide right
Step 3:       [2, 4, 5] ← Slide right
Step 4:          [4, 5, 6] ← Slide right
Step 5:             [5, 6, 7] ← Slide right

There are two types of sliding window:
1. Fixed size window - window is constant
2. Variable size window - the window shrinks and grows based on the problem constraints

## Fixed size sliding window

You would use this when:
1.  You have a max and minimum in each window of size k
2. You want the average of the window of size k
3. Pattern matching e.g. substrings

## Variable size sliding (two pointers)

You would use this when you have a sub array with sum/ product/ or a condition
Longest or shorting array meeting criteria
String problems - longest substring without repeating characters

## Example problems


### Fixed size window problems
1. Maximum average subarray of size k
2. Anagrams in a string
3. Maximum time in each time window

### Variable size window problems
1. Rate limiting - N requests per T seconds, window - time based sliding approach, queue of timestamps per user `../hackerrank/src/one_hour_tests/write_a_rate_limiter.rs`
2. Minimum size subarray sum
3. Longest substring with a most k distinct characters 


Complexity analysis:

## Fixed size window complexity analysis
Bets case 0 - single pass through the array - O(n), visiting each element once
Average case - O(n) - where n is the number of elements processed once
Worst case - O(n *k) where k is the window size

## Variable size window complexity analysis
Best case - O(n) where each element is visited twice (left and right)
Average case  - O(n) - amortized pointer (the average time per operation over a sequence of operations), so some operations are more expensive than others but on average they are balanced out

It is a primarily a data structure problem not an algorithm pro

TODO: Link hackerrank questions for these problems and files to these
*/

    // Fixed size sliding window:
    // 1. Initialize: left = 0, right = 0
    // 2. Expand window to size k (move right pointer)
    // 3. Process first window
    // 4. While right < array.length:
    //      a. Remove left element from window
    //      b. Add right element to window
    //      c. Move left and right pointers right by 1
    //      d. Process current window
    // 5. Done
fn fixed_size_sliding_window(){
    todo!();
}

// 1. Initialize: left = 0, right = 0
// 2. While right < array.length:
//      a. Expand window (move right pointer)
//      b. Add right element to window
//      c. Update window state
//      d. While window violates condition:
//           - Shrink window (move left pointer)
//           - Remove left element from window
//           - Update window state
//      e. Check if current window is valid solution
//      f. Update best answer if needed
// 3. Return best answer
fn variable_size_sliding_window(){
    todo!();
}

