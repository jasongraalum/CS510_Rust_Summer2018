///Copyright (c) 2018 Jason Graalum
///
/// CS510 Rust Programming
/// Summer 2018
///
/// HW #3
///

// Enum for WordProblem state machine
enum Op {
    Start,
    Plus,
    Minus,
    Multiplied,
    Divided,
    Next,
    Error,
}

// Simple struct holding WordProblem result
pub struct WordProblem<'a> {
    result: Result<i32, &'a str>,
}

// WordProblem implementation
//      new -> parse and return the result of the evaluation
//      is_err() -> returns false if the result is an error
//      answer() -> if result is of Ok() type, return the value
//
impl<'a> WordProblem<'a> {
    pub fn new(problem: &'a str) -> Self {
        println!("{}", problem);

        //
        // big harry function
        // 1. Remove the ? from the end
        // 2. Split by white space and filter "What", "is" and "by" <- remove by from multiplied by
        // 3. fold the remaining elements through the state machine
        //
        let result = problem
            .trim_right_matches('?')
            .split_whitespace()
            .filter(|s| s != &"by" && s != &"What" && s != &"is")
            .fold((0, Op::Start), |(acc, curr_op), val| match curr_op {
                Op::Error => (0, Op::Error),
                Op::Next => match val {
                    "plus" => (acc, Op::Plus),
                    "minus" => (acc, Op::Minus),
                    "divided" => (acc, Op::Divided),
                    "multiplied" => (acc, Op::Multiplied),
                    _ => (0, Op::Error),
                },
                Op::Start => match val.parse::<i32>() {
                    Err(_) => (0, Op::Error),
                    Ok(v) => (v, Op::Next),
                },
                Op::Plus => match val.parse::<i32>() {
                    Err(_) => (0, Op::Error),
                    Ok(v) => (acc + v, Op::Next),
                },
                Op::Minus => match val.parse::<i32>() {
                    Err(_) => (0, Op::Error),
                    Ok(v) => (acc - v, Op::Next),
                },
                Op::Multiplied => match val.parse::<i32>() {
                    Err(_) => (0, Op::Error),
                    Ok(v) => (acc * v, Op::Next),
                },
                Op::Divided => match val.parse::<i32>() {
                    Err(_) => (0, Op::Error),
                    Ok(v) => (acc / v, Op::Next),
                },
            });

        // Return a WordProblem depending on if the result returned a (i32, _) tuple of a (_,
        // Op::Error) tuple
        match result {
            (_, Op::Error) => WordProblem {
                result: Err("Bad command"),
            },
            (val, _) => WordProblem { result: Ok(val) },
        }
    }

    //
    // Return the result value
    //
    pub fn answer(&self) -> Result<i32, &'a str> {
        self.result
    }

    //
    // Return true if evaluation resulted in error
    //
    pub fn is_err(&self) -> bool {
        match self.result {
            Err(_) => true,
            Ok(_) => false,
        }
    }
}
