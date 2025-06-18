use std::io::{self, BufRead};
use std::fmt;

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 
 Simple solution:
 1. Loop through array, 
    a. if postive increment positive counter
    b. if negative increment negative counter
    c. if zero increment positive counter
2. Create ratios
3. For each value, create it as a float

Complex solution:
    
    
    3. Implement display for custom type that has a to string implementation for 
 */
 
/// Tuple struct for value rations
/// 0 = positive
/// 1 = 0 vals
/// 2 = negative
/// 4 = size of array
pub struct ValueRatios(i32, i32, i32, i32);

impl fmt::Display for ValueRatios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}\n{}", (self.0 as f32 / self.3 as f32), (self.2 as f32/ self.3 as f32 ), (self.1 as f32/ self.3 as f32))
    } 
}

fn plusMinus(arr: &[i32], value_ratio_counter: &mut ValueRatios) {
    
    for &value in arr.iter() {
        if value >= 1 {
            value_ratio_counter.0+=1;
        } else if value == 0 {
            value_ratio_counter.1+=1;
        } else if value < 0 {
            value_ratio_counter.2+=1;
        } else {
            panic!("unknown circumstance found");
        }
    }
    
    println!("{}", value_ratio_counter);

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
        
    let value_ratio_counter = &mut ValueRatios(0, 0, 0, n);


    plusMinus(&arr, value_ratio_counter);
}
