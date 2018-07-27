
enum Op {
    Start,
    Plus,
    Minus,
    Multiplied,
    Divided,
    Next,
    Error,
}


pub struct WordProblem<'a> {
    result: Result<i32, &'a str>,
}

impl<'a> WordProblem<'a> {
    pub fn new(problem : &'a str) -> Self {
        println!("{}", problem);

        //
        // Ex: "What is -1 plus-10?"
        //
        // Remove the ? from the end;
        //
        let result = problem.trim_right_matches('?')
                .split_whitespace()
                .filter(|s| s != &"by" && s != &"What" && s != &"is")
                .fold((0, Op::Start), |(acc, curr_op), val| 
                      match curr_op {
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
                      }
                     ); 


                     match &result.1 {
                         &Op::Error => WordProblem { result : Err("Bad command") }, 
                         _ => WordProblem { result : Ok(result.0) }, 
                     }
    }

    pub fn answer(&self) -> Result<i32, &'a str> {
        self.result
    }

    pub fn is_err(&self) -> bool {
        match self.result {
            Err(_) => true,
            Ok(_) => false,
        }
    }



}

