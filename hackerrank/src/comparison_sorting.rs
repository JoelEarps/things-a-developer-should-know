use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
/*
 * Complete the 'countingSort' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

/// Initial solution (and thoughts)
/// Return is always 100, so create a vec with capacity 100
/// As you increment through, get value at index, then set
fn countingSort(arr: &[i32]) -> Vec<i32> {
    // Always returns an array of 100 elements i.e. max value is 99
    // TODO: Code smell add proper type casting checks and panic
    let mut sorted_vec = vec![0; 100];

    // Loop through slice and increment the value based on this
    // Can be 0 therefore 0 indexing valid
    for &value in arr {
        match usize::try_from(value) {
            Ok(index) if index < sorted_vec.len() => sorted_vec[index] += 1,
            _ => panic!("Invalid value: {}", value),
        }
        // sorted_vec[value as usize] += 1
    }

    sorted_vec
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = countingSort(&arr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
