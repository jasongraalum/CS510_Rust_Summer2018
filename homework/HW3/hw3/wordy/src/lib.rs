
enum Command {
    Plus,
    Minus,
    Multiplied,
    Divided,
}

struct CommandStack<'a> {
    l_arg: i32,
    command: Option<Command>,
    r_exp: &'a Option<CommandStack<'a>>,
}

impl<'a> CommandStack<'a> { 
    pub fn build_command(input : &'a str) -> Result<i32,&str>  {
        // Legal Cases
        // l_val == i32, command = None
        // l_val == i32, command = Some<Command>, r_exp = Some(String)
        //
        // Any other combination is an error
        // Error Cases
        // l_val == None
        // l_val => Not an i32
        //
        // command != None and r_exp == None
       
        // Split the input string into: l_val, command, r_exp 
        let mut input_vec : Vec<&str> = input.splitn(3,' ').collect();

        // Must be at least one value which is the lhs
        // Checks for l_val == None and l_val != i32
        let l_val : i32 = match input_vec.pop() {
            None => return Err("No l-arg"),
            Some(arg) => { 
                match arg.parse() {
                    Ok(l_arg) => l_arg,
                    Err(_) => return Err("Invalid lhs value"),
                }
            }
        };

        // If command == None, return a CommandStack with only a l_arg
        let command = match input_vec.pop() {
            None => return Ok(l_val),
            Some(cmd) => match cmd { 
                "plus" => Some(Command::Plus),
                "minus" => Some(Command::Minus),
                "multiplied" => Some(Command::Multiplied),
                "divided" => Some(Command::Divided),
                _ => return Err("Not a valid operation"),
            },
        };

        let result = match input_vec.pop() {
            None => return Err("Missing r-arg"),
            Some(r_exp_str) => match CommandStack::build_command(r_exp_str) {
                Err(_) => return Err("Invalid r-exp"),
                Ok(r_val) => match command {
                    Some(Command::Plus) => l_val + r_val,
                    Some(Command::Minus) => l_val - r_val,
                    Some(Command::Multiplied) => l_val * r_val,
                    Some(Command::Divided) => l_val / r_val,
                    None => return Err("Invalid r-exp"),
                },
            }
        };
        return Ok(result);
    }
}


pub struct WordProblem<'a> {
    result: Result<i32, &'a str>,
}

impl<'a> WordProblem<'a> {
    pub fn new(problem : &'a str) -> Self {
        //"What is -1 plus-10?"
        //
        // Remove the ? from the end;

        let mut command_vec : Vec<&str> = problem.splitn(3,' ').collect();

        if command_vec.len() < 3 { 
            return WordProblem { result :Err("Invalid number of arguments") }; 
        }

        // Get rid of What and is
        let _ = match command_vec.pop() {
            None => return WordProblem { result:  Err("Invalid command str") },
            Some(s) => s,
        };
        let _ = match command_vec.pop() {
            None => return WordProblem { result:  Err("Invalid command str") },
            Some(s) => s,
        };
        
        match command_vec.pop() {
            None => WordProblem { result:  Err("Invalid command str") },
            Some(c) => WordProblem { result : CommandStack::build_command(c) }
        }

    }

    pub fn answer(&self) -> Result<i32, &'a str> {
        self.result
    }


}

