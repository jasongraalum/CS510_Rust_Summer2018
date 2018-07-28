/// Copyright (c) 2018 Jason Graalum
///
///
pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    let normalized: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c)
        .collect();

    let rows = (normalized.len() as f64).sqrt().ceil() as usize;
    let cols = (normalized.len() as f64).sqrt().floor() as usize;

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

    for s in out.iter_mut().take(rows - 1) {
        s.push(' ');
    }

    let mut code: String = String::new();
    for s in &out {
        code.push_str(&s);
    }
    code
}
