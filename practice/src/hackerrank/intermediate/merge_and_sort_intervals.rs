/// Merge Overlapping Intervals
/// 
/// Algorithm:
/// 1. Sort intervals by start time
/// 2. Iterate through, comparing each interval with the last merged one
/// 3. If they overlap (current.start <= last.end), merge by extending last.end
/// 4. If no overlap, push current interval to result
/// 
/// Time Complexity: O(n log n) - dominated by sorting
/// Space Complexity: O(n) - for the result vector

pub fn merge_intervals(mut intervals: Vec<[i64; 2]>) -> Vec<[i64; 2]> {
    if intervals.is_empty() {
        return vec![];
    }

    // Step 1: Sort by start time
    intervals.sort_by_key(|interval| interval[0]);

    // Step 2: Initialize result with first interval
    let mut merged: Vec<[i64; 2]> = vec![intervals[0]];

    // Step 3: Process remaining intervals
    for current in intervals.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();

        // Check if current overlaps with last merged interval
        // Overlap condition: current start <= last end
        if current[0] <= last[1] {
            // Merge: extend the end to the max of both
            last[1] = last[1].max(current[1]);
        } else {
            // No overlap: add as new interval
            merged.push(current);
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let intervals = vec![[1, 3], [2, 6], [8, 10], [15, 18]];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn test_unsorted_input() {
        let intervals = vec![[8, 10], [1, 3], [15, 18], [2, 6]];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn test_all_overlapping() {
        let intervals = vec![[1, 4], [2, 5], [3, 6]];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![[1, 6]]);
    }

    #[test]
    fn test_no_overlapping() {
        let intervals = vec![[1, 2], [4, 5], [7, 8]];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![[1, 2], [4, 5], [7, 8]]);
    }

    #[test]
    fn test_contained_interval() {
        // [2, 3] is fully contained within [1, 5]
        let intervals = vec![[1, 5], [2, 3]];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![[1, 5]]);
    }

    #[test]
    fn test_empty() {
        let intervals: Vec<[i64; 2]> = vec![];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![] as Vec<[i64; 2]>);
    }

    #[test]
    fn test_single_interval() {
        let intervals = vec![[1, 5]];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![[1, 5]]);
    }

    #[test]
    fn test_touching_intervals() {
        // [1, 3] and [3, 5] touch at 3 - they should merge
        let intervals = vec![[1, 3], [3, 5]];
        let result = merge_intervals(intervals);
        assert_eq!(result, vec![[1, 5]]);
    }
}

