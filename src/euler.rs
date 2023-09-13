// Estimation methods for the Euler Number (e)
// See https://en.wikipedia.org/wiki/E_(mathematical_constant)

// Estimate e with series: 1/1 + 1/1 + 1/(1*2) + 1/(1*2*3) + ...
fn euler_series(n_terms: u32) -> f64 {
    // TODO: implement this function
}
   

// Estimate e based on the limit formula: (1 + 1/n) ^ n
fn euler_limit(n_limit: u32) -> f64 {
    // TODO: implement this function
}

// You can use the main function as your playground
// Feel free to edit/use it for experimenting with your code
// The main function will not be tested/graded in this file
fn main() {
    println!("euler_series(10) = {}", euler_series(10));
    println!("euler_limit(1_000) = {}", euler_limit(1_000));
}

//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    const REL_TOL: f64 = 1e-9;

    #[test]
    fn test_euler_series() {
        assert_relative_eq!(euler_series(1), 1.0, max_relative=REL_TOL);
        assert_relative_eq!(euler_series(2), 2.0, max_relative=REL_TOL);
        assert_relative_eq!(euler_series(10), 2.7182815255731922, max_relative=REL_TOL);
        assert_relative_eq!(euler_series(100), 2.7182818284590455, max_relative=REL_TOL);
    }

    #[test]
    fn test_euler_limit() {
        assert_relative_eq!(euler_limit(1), 2.0, max_relative=REL_TOL);
        assert_relative_eq!(euler_limit(1_000), 2.7169239322355936, max_relative=REL_TOL);
        assert_relative_eq!(euler_limit(1_000_000), 2.7182804690957534, max_relative=REL_TOL);
    }
}
