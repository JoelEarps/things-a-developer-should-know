// Problem context
// https://spin.atomicobject.com/learning-rust-binary-search/

/* 
Binary search works on a sorted list of data and reduces the amount the search area each iteration
It works by comparing the middle value of the array against the value being searched for. 
If the value in the middle is equal to the searched value, it returns the correct value. 
If the value being searched for is higher than the middle of the array, 
it returns the higher half of the list. This is vice versa for the lower half of the list.
It does this repeatedly reducing the search area until the value is found, or it reaches either extreme.
There are three examples below:
1. Implementation of the raw binary search
2. Binary search with idiomatic rust
3. Using the binary search function available in the std lib
 */

#[derive(Debug)]
struct TradeFee {
    volume: usize, 
    total_price: usize
}

type UserTradeFees = Vec<TradeFee>;

// The problem arose when I was trying to look for where a trade amount fit in when looking for associated prices
fn find_trade_amount(trade_fees: &[TradeFee], target_volume_for_placement: &usize) -> Option<usize> {
    if trade_fees.is_empty() {
        return None;
    }

    let mut lower_bound = 0;
    let mut higher_bound = trade_fees.len() - 1;
    let mut result: Option<usize> = None;

    while lower_bound <= higher_bound {
        
        println!("The Lower bounds (index) of the search area {}, is currently less than or equal to the higher bounds (index) of the search area ({})", lower_bound, higher_bound);
        let mid = (lower_bound + higher_bound) / 2;
        println!("The mid point of the search vec is at index: {} ", mid);
        let current_mid_volume_under_search = &trade_fees[mid].volume;
        println!(
            "Is target volume {}, larger than the current volume at the mid point of the vector being searched {}",
            target_volume_for_placement, current_mid_volume_under_search
        );
        if target_volume_for_placement >= &current_mid_volume_under_search {
            println!("Yes it is, setting the current result is bigger than the higher bound, setting the new low bound to be the index after the current midpoint, therefore new search area is {} to {}", mid + 1 , higher_bound);
            let value = &trade_fees[mid].volume;
            result = Some(value.clone());
            lower_bound = mid + 1;
        } else {
            if mid == 0 {
                println!("The mid point of the search area is 0, therefore we cannot search anymore, therefore the volume should be placed at the start of the array");
                let value = &trade_fees[mid].volume;
                result = Some(value.clone());
                break;
            }
            println!("No it is not, setting the new higher bound to be one below the current middle index, therefore new search area is {} to {}", lower_bound, mid -1);
            higher_bound = mid - 1;
        }
    }
    println!("We have reached the end of the binary search algorithm (where either the lower bound is now larger than the higher bound, meaning we place the , the end volume is {:?}", result);
    // The output here is contextual to the search
    // 1. The output is the volume 
    // 2. If the output volume is the same as the first index volume, then we insert at the start of the input array
    // 3. If the output volume is 
    // To handle this you could return an enum where it tells you where to insert - between to values, at the start or at the end
    result
}

fn find_vip_tier_rust_idiomatic(){}

fn find_vip_tier_with_rust_binary_search_function(){}


fn find_vip_tier_binary_search_by_key(){}

#[cfg(test)]
mod binary_search_tests {

    static test_struct: &[TradeFee] = &[TradeFee {
        volume: 1,
        total_price: 100
    },
    TradeFee {
        volume: 3,
        total_price: 100
    },
    TradeFee {
        volume: 5,
        total_price: 100
    },
    TradeFee {
        volume: 7,
        total_price: 100
    }
    ];

    use super::*;
    
    #[test]
    fn raw_binary_search(){
        // Could use RSTest here, but not the point right now, maybe add at the end
        let target_volume_1: usize = 6;
        let entry_volume_to_be_inserted_after = find_trade_amount(test_struct, &target_volume_1);
        assert!(entry_volume_to_be_inserted_after.is_some());
        assert_eq!(entry_volume_to_be_inserted_after.unwrap(), 5);
        let target_volume_1: usize = 0;
        let entry_volume_to_be_inserted_after = find_trade_amount(test_struct, &target_volume_1);
        assert!(entry_volume_to_be_inserted_after.is_some());
        assert_eq!(entry_volume_to_be_inserted_after.unwrap(), 1);
        let target_volume_1: usize = 10;
        let entry_volume_to_be_inserted_after = find_trade_amount(test_struct, &target_volume_1);
        assert!(entry_volume_to_be_inserted_after.is_some());
        assert_eq!(entry_volume_to_be_inserted_after.unwrap(), 7);
    }
}
