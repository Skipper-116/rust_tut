fn main() {
    first_function();
    second_function();
}

fn second_function() {
    // this will be dealing with datatypes
    // first thing will be scalar types
    // integers
    let x = -6;
    printer(x);
    let x: f64 = -10.09;
    floating_print(x);
    let x: bool = true;
    boolean_print(x);
    let x: char = 'a';
    char_print(x);

    // Time to play with compound types
    let mut tup = (500, 6.4, 1);
    tup.0 = -500;
    printer(tup.0);
    let mut arr = [1, 2, 3, 4, 5];
    arr[0] = 10;
    printer(arr[0]);
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

fn floating_print(x: f64) {
    println!("x is: {}", x);
}

fn boolean_print(x: bool) {
    println!("x is: {}", x);
}

fn char_print(x: char) {
    println!("x is: {}", x);
}