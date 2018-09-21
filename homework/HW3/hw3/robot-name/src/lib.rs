/// Copyright (c) 2018 Jason Graalum
///
/// CS510 Rust Programming
/// Summer 2018
///
/// HW #3
/// Generate a random(but NOT UNIQUE) name for a Robot
///
extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Robot {
    name: String,
}

// Simple use of gen_range function to create a random name of
// Char/Char/3-digits
//
fn gen_name() -> String {
    let mut rng = thread_rng();
    let id_char1 = (rng.gen_range(0, 25) as u8 + 65) as char;
    let id_char2 = (rng.gen_range(0, 25) as u8 + 65) as char;
    let id_num = rng.gen_range(100, 1000);

    let mut new_name = String::new();
    new_name.push(id_char1);
    new_name.push(id_char2);
    new_name.push_str(&id_num.to_string());

    new_name
}

// Robot structure methods
impl Robot {
    pub fn new() -> Robot {
        Robot { name: gen_name() }
    }
    // Return the possibly non-unique robot name
    pub fn name(&self) -> &str {
        &self.name
    }

    // Reset the robot name and generate another randon, but possibily non-unique name
    pub fn reset_name(&mut self) {
        self.name = "".to_string();
        self.name = gen_name();
    }
}
