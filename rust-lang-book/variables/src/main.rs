fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: i32 = 100_000;
    println!("{}", SUBSCRIBER_COUNT);

    // Compound Types
    let tup = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    println!("channel: {}, sub_count {}", channel, sub_count);

    let error_codes = [200, 304, 500];
    let not_found = error_codes[1];
    let byte = [0; 8];
    println!("error code not_found: {}, others code errors: {:?}", not_found, error_codes);
    println!("byte : {:?}", byte);

    let result = my_function(11, 22);
    println!("The function result is {}", result);

    // control flow
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value number is {} when condition is {}", condition, number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result is {}", result);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}