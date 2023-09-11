// Functions in Rust

// TODO: Add the missing pieces to this function 
fn is_prime(n) {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i {
            return false;
        }
    }
}

// TODO: Add the missing pieces to this function 
fn factorial(n) {
    let product = 1.0;
    for i in 1..=n {
        product *= i;
    }
}

// You can use the main function as your playground
// Feel free to edit/use it for experimenting with your code
// The main function will not be tested/graded in this file
fn main() {
    println!("factorial(6) = {}", factorial(6));
    println!("is_primt(42) = {}", is_prime(42));
}

//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_factorial() {
        assert_relative_eq!(factorial(1), 1.0);
        assert_relative_eq!(factorial(6), 720.0);
        assert_relative_eq!(factorial(13), 6227020800.0);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(7));
        assert!(!is_prime(14));
        assert!(is_prime(37));
        assert!(!is_prime(42));
    }
}
