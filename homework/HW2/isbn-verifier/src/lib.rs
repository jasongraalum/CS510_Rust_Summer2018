/// Copyright (c) 2018 Jason Graalum
///
/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum: i32 = 0;
    isbn.replace("-", "")
        .char_indices()
        .for_each(|(i, c)| {
        if sum < 0 || i > 9 || (i < 9 && !c.is_digit(10)) {
            sum = -1;
        } else if c.is_digit(10) {
            sum += (c.to_digit(10).unwrap() as i32) * (10 - (i as i32));
        } else if c == 'X' {
            sum += 10 * (10 - (i as i32));
        } else {
            sum = -1;
        }
    });
    sum % 11 == 0
}
