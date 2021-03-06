/// Copyright (c) 2018 Jason Graalum
///
/// Function to emulate the sound the sounds raindrops
/// make when they are factoring
///
pub fn raindrops(n: usize) -> String {
    let mut s = String::new();
    if n % 3 == 0 {
        s.push_str("Pling");
    }
    if n % 5 == 0 {
        s.push_str("Plang");
    }
    if n % 7 == 0 {
        s.push_str("Plong");
    }
    if s.is_empty() {
        return n.to_string();
    }
    s
}
