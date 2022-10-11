fn main() {
    first_function();
    second_function();
}

fn second_function() {
    // this will be dealing with datatypes
    // first thing will be scalar types
    // integers
    let _x = -6;
    println!("Second function.");
}

fn first_function() {
    // negative numbers
    const SOME_WEIRD_AMOUNT: i64 = -922337203608;
    printer(SOME_WEIRD_AMOUNT);
    let x = 4; // x: i32
    printer(x);
    {
        let x = x - 1; // x: i32
        printer(x);
    }
    let x = 5;
    printer(x);
    printer(x + 1);
}

fn printer(x: i64) {
    println!("x is: {}", x);
}
