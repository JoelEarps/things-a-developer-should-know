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


struct TradeFee {
    volume: usize, 
    total_price: usize
}

pub type UserTradeFees = Vec<TradeFee>;


// The problem arose when I was trying to look for where a trade amount fit in
fn find_trade_amount(tiers: UserTradeFees, target_volume: usize) -> Option<usize> {
    if tiers.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = tiers.len() - 1;
    let mut result: Option<usize> = None;

    while low <= high {
        println!("Low is : {}, high is :{} ", low, high);
        let mid = (low + high) / 2;
        println!("Mid is : {} ", mid);
        let ntl_cutoff = &tiers[mid].ntl_cutoff
        println!(
            "Is target volume {}, larger than {}",
            target_volume, ntl_cutoff
        );
        if target_volume >= &ntl_cutoff {
            println!("{:?}", tiers[mid]);
            let value = &tiers[mid].volume;
            result = Some(value);
            low = mid + 1;
        } else {
            if mid == 0 {
                tracing::warn!("Trade volume is the lower than the value required for the minimum VIP level");
                result = Some(&tiers[mid].volume);
                break;
            }
            high = mid - 1;
        }
    }
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
    
    fn raw_binary_search(){

    }
}
