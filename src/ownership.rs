// Ownership in Rust

fn squares(n: u32) -> Vec<f64> {
    let mut values = Vec::new();
    for i in 0..n {
        values.push(i as f64 * i as f64);
    }
    values
}

fn median(values: Vec<f64>) -> f64 {
    let mut values = values;
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = values.len();
    if (mid % 2) != 0 {
        values[mid / 2]
    } else {
        values[mid / 2 - 1] + values[mid / 2] / 2.0
    }
}
fn main() {
    let mut values = squares(14);
    let median_value = median(values);
    // TODO: make this line work with minimal changes
    // Note: you should be able to print the values and their mean_value
    // Hint: change the way median() receives and works with tht values
    println!("The median of {:?} is {}", values, median_value);
}
