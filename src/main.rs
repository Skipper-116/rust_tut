use std::string;

fn main() {
    // negative numbers
    const SOME_WEIRD_AMOUNT: u64 = 922337203608;
    printer(SOME_WEIRD_AMOUNT);
    let  x = 4; // x: i32  
    printer(x);
    {
        let  x = x - 1; // x: i32
        printer(x);
    }
    let x = 5;
    printer(x);
    printer(x + 1);
}

fn printer(x: u64) {
    println!("x is: {}", x);
}

