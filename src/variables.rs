// Variables in Rust

fn shadow_variable() {
    let x = 5;
    println!("Before change {}", x);
    let x = "seven";
    println!("After change {}", x);
}

fn mutable_variable() {
    let mut x = 5;
    println!("Before change {}", x);
    x += 8;
    println!("After change {}", x);
}

fn basic_variable() {
    let x = 42;
    println!("My favorite number is: {}", x);
}

fn main() {
    basic_variable();
    mutable_variable();
    shadow_variable();
}
