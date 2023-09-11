// Basic Types in Rust

fn arrays() {
    let arr = [1, 2, 3, 4, 5];

    // TODO: finish the next line (please, do not hardcode 5)
    println!("Array length is {}", );
}

fn tuples() {
    let person = ("Peter", 47);

    // TODO: use the tuple fields to print the results
    println!("{} is {} years old.", );
}

fn methods() {
    // TODO: understand and fix the problem in this function
    let x = -12;
    println!("The absolute value of x^3: {}", x.pow(3).abs());
}

fn chars() {
    let ch = '7';

    if // TODO: finish this line, so that the program compiles and behaves as expected
        println!("ch is a decimal digit");
    } else {
        println!("ch is NOT a decimal digit");
    }
}

fn bools() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // TODO: declare and initialize the `is_evening` variable
    if is_evening {
        println!("Good evening!");
    }
}

fn numbers() {
    let x = 13;
    let y = 2.0;
    // TODO: fix this line to compile
    let z = x * y;

    println!("{z}");
}

fn main() {
    numbers();
    bools();
    chars();
    methods();
    tuples();
    arrays();
}
