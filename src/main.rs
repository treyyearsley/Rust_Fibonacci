fn fibonacci(n: u32) -> u32 {
    //create fibonacci sequence
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

//run the fibonacci sequence funtion with several different numbers
fn main() {
    println!("Fibonacci Sequence");
    println!("{}", fibonacci(0));
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(5));
    println!("{}", fibonacci(6));
    println!("{}", fibonacci(7));
    println!("{}", fibonacci(8));
    println!("{}", fibonacci(9));
}
