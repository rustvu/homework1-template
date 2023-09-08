// Estimation methods for the Euler Number (e)
// See https://en.wikipedia.org/wiki/E_(mathematical_constant)

// Estimate e with series: 1/1 + 1/1 + 1/(1*2) + 1/(1*2*3) + ...
fn euler_series(n_terms: u32) -> f64 {
    let mut e = 1.0;
    let mut denominator = 1.0;
    for i in 1..n_terms {
        denominator *= i as f64;
        e += 1.0 / denominator;
    }
    e
}
   

// Estimate e based on the limit formula: (1 + 1/n) ^ n
fn euler_limit(n_limit: u32) -> f64 {
    let n = n_limit as f64;
    (1.0 + 1.0 / n).powf(n)
}

// You can use the main function as your playground
fn main() {
    // println!("euler_series(10) = {}", euler_series(10));
}


// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_euler_series() {
        assert_relative_eq!(euler_series(1), 1.0);
        assert_relative_eq!(euler_series(2), 2.0);
        assert_relative_eq!(euler_series(10), 2.7182815255731922);
        assert_relative_eq!(euler_series(100), 2.7182818284590455);
    }

    #[test]
    fn test_euler_limit() {
        assert_relative_eq!(euler_limit(1), 2.0);
        assert_relative_eq!(euler_limit(1_000), 2.7169239322355936);
        assert_relative_eq!(euler_limit(1_000_000), 2.7182804690957534);
    }
}