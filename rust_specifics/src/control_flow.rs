fn mock_if_statement()-> bool {
    let fake_num = 3;
    if fake_num == 3 {
        true
    } else if fake_num > 3 {
        false
    } else {
        println("Invalid fake number");
        false
    }
}

fn ternary_in_rust() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn for_in_loop(){

    let a = [0;10];
    for item in a {
        println("Iterating, value is {item}");
    }

}

fn while_loop(){
    let mut counter = 0;
    let total = 5;
    while counter < total {
        println("Counted {counter} times");
        counter+=1;
    }
}

fn loop_with_labels(){
    let mut count = 0;
   let test_loop_return = 'test_counter: loop {
    println!("count = {count}");
    let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'test_counter;
            }
            remaining -= 1;
        }
        count += 1;
    };
        println!("End count = {count}");
}
