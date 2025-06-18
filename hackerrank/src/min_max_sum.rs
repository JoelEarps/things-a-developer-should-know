use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

/// This is a classic problem where the following is the way to do it?
/// i64 problem - remember this
fn miniMaxSum(arr: &[i32]) {
    let mut sum: i64 = 0;
    let mut min = arr[0];
    let mut max = arr[0];

    for &value in arr.iter() {
        sum += value as i64;
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }

    println!("{} {}", sum - max as i64, sum - min as i64);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
