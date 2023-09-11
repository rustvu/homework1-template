// Variables in Rust

fn shadow_variable() {
    let x = 5;
    println!("Before change {}", x);
    // TODO: fix the next line, so the function compiles (do not modify other lines in this function)
    x = "seven";
    println!("After change {}", x);
}

fn mutable_variable() {
    // TODO: fix the next line, so the function compiles (do not modify other lines in this function)
    let x = 5;
    println!("Before change {}", x);
    x += 8;
    println!("After change {}", x);
}

fn basic_variable() {
    // TODO: add the necessary variable declaration/initialization
    println!("My favorite number is: {}", x);
}

fn main() {
    basic_variable();
    mutable_variable();
    shadow_variable();
}
