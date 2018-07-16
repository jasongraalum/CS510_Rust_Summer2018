///
/// Copyright (c) 2018 Jason Gralaum
///
///
/// Find the Pythagorean Triplet for
/// a + b + c = 1000
//
pub fn find() -> Option<u32> {
    for a in 1..500 {
        for b in 1..500 {
            let c = 1000 - a - b;
            if c * c == a * a + b * b {
                return Some(c * b * a);
            }
        }
    }
    None
}
