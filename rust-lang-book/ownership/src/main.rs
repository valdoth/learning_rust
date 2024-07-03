fn main() {
    println!("Hello, world!");

    fn a() {
        let x = "hello";
        let y = 22;
        b(x);
        println!("y is {}", y);
    }

    fn b(x: &str) {
        let y = String::from("world");
        println!("x is {} and y is {}", x, y);
    }

    a();
}
