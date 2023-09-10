// Basic Types in Rust

fn arrays() {
    let arr = [1, 2, 3, 4, 5];

    println!("Array length is {}", arr.len());
}

fn tuples() {
    let person = ("Peter", 47);

    println!("{} is {} years old.", person.0, person.1);
}

fn methods() {
    let x = -12i32;
    println!("The absolute value of x^3: {}", x.pow(3).abs());
}

fn chars() {
    let ch = '7';

    if ch.is_digit(10) {
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

    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}

fn numbers() {
    let x = 13;
    let y = 2.0;
    let z = x as f64 * y;

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
