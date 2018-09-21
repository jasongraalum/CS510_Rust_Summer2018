/// Copyright (c) 2018 Jason Graalum
///
/// CS510 Rust Programming
/// Summer 2018
///
/// HW #3
/// Encrypt a string with square encoding
///
pub fn encrypt(input: &str) -> String {
    // Sanity check - return nothing if nothing is given
    if input.is_empty() {
        return "".to_string();
    }

    // Normalize by changing to lower case and removing anything but alpha-numerics
    let normalized: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c)
        .collect();

    // Calculate the size of the square
    let rows = (normalized.len() as f64).sqrt().ceil() as usize;
    let cols = (normalized.len() as f64).sqrt().floor() as usize;

    // Build a Vec of Strings one string at a time by accessing the
    // normalized string out of order
    let mut out: Vec<String> = Vec::with_capacity(rows);
    for i in 0..rows {
        let mut s = String::new();
        for j in 0..cols {
            if rows * j + i < normalized.len() {
                s.push(normalized[rows * j + i])
            } else {
                s.push(' ');
            }
        }
        out.push(s);
    }

    // Add a space to the end of each String - expect the last one.
    for s in out.iter_mut().take(rows - 1) {
        s.push(' ');
    }

    // Build the final String from each of the Strings in the out Vector
    let mut code: String = String::new();
    for s in &out {
        code.push_str(&s);
    }
    code
}
