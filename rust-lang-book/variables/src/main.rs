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
}
