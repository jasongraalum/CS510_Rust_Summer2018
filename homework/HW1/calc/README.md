# CS510 Rust Programming
# Homework #1

## Simple Calculator Program
## Student: Jason Graalum
## Date: June 28, 2018

### Development Process(What I Did)
I wrote the calc program as a single rust "--bin" project. I did not create a separate library or module since the end result was to be a single executable, command-line program. I followed the steps below when developing the program. I decided to use a Test-Driven-Development methodology by first writing tests for each function - followed by the function.

1. Built new rust program with cargo
   ```
   cargo new --bin calc
   ```
1. Created function templates for main and each calculator function.
   - All functions return 0 until implemented
   ```
   fn sum(mut vals: Vec<i64>) -> i64 { 0 }
   fn product(mut vals: Vec<i64>) -> i64 { 0 }
   fn gcd(mut vals: Vec<i64>) -> i64 { 0 }
   fn lcm(mut vals: Vec<i64>) -> i64 { 0 }
   ```
1. Add simple test section for each function.
   - For the first pass of the tests, all tests expect '0';
   ```
   fn test_sum() {
       assert_eq!(sum(), 0);
   }
   ```
1. Run tests 
   - Validate the program structure is correct. 
   - At this point, the main function is still empty.  
1. Add function argument types(Vec<i64>) and update tests
   ```
   fn test_sum() {
       assert_eq!(sum(vec![]), 0);
       assert_eq!(sum(vec![10]), 0);
       assert_eq!(sum(vec![3, 10]), 0);
   }
   ```
1. Add function implementations 
   - Sum and product are done in simple "functional" style.
   - The GCD command does a recursive call to handle the case where more than 2 values are provided.
   - The LCM calls the GCD function and is also recursive if more than 2 values are provided.
   - Replace test expect values with correct values and add more tests.
   ```
   fn test_sum() {
       assert_eq!(sum(vec![]), 0);
       assert_eq!(sum(vec![10]), 10);
       assert_eq!(sum(vec![3, 10]), 13);
       assert_eq!(sum(vec![1, 2, 3]), 6);
       assert_eq!(sum(vec![1, 2, 3, 4]), 10);

       assert_eq!(sum(vec![-10]), -10);
       assert_eq!(sum(vec![-3, 10]), 7);
       assert_eq!(sum(vec![-1, 2, -3]), -2);
       assert_eq!(sum(vec![-1, -2, -3, -4]), -10);
   }

   //
   // Return sum of values
   //
   fn sum(vals: Vec<i64>) -> i64 {
       vals.iter().fold(0, |n, i| n + i)
   }
   ```
1. Since some of the functions don't accept values of '0', the functions need to "panic" and return.  Add this functionality as well as the tests. The lcm panic tests are shown below:
   ```
   #[test]
   #[should_panic]
   fn test_lcm_fails() {
       lcm(vec![0, 10]);
       lcm(vec![20, 0, 10]);
   }
   ```
1. Create main function
   - The command line arguments needed to be parsed. The first and second arguments are the program name and the initial command.  The remaining arguments are the values to be operated on.  We convert these from a Vector of Strings to a Vector of i64 type with:
   ```
   // Parse args to i64
   let args: Vec<i64> = cmd_args
       .into_iter()
       .map(|n| str::parse::<i64>(&n).unwrap())
       .collect();
   ```
   - Next, a match command is added to call the appropriate function based on the command provided. On caveat was that the case where no values are provided needs to be checked before the previous parsing.
1. Add Usage and Version options
   - Finally, I added options to the program to print a usage message or a version message given the -h/--help or -v/--version commands.
   - I tried initially to add these to the match command which parses the function commands. However, since there is no return value for these, I decided to use a separate match command which has no return.
1. Generate rust documentation
   - I updated the comments to include some "crate" information, but since there are no dependencies and this isn't a library, the result is a bit boring.
   ```
   cargo doc --nodeps --open
   ```

### Summary(How It Went)
There were a few issues I had to overcome:
1. Parsing the command line arguments
   - Getting the command was easy enough. Since I could "destroy" the input vector, I just removed the value from the front of the vector
   - Parsing the values from a Vector of Strings into a Vector of i64 was a bit more difficult. I wanted to use function style, so I used the following:
     - into_iter() to get an iterator of the vector. My understanding is that into_iter() moves the vector elemnts into the iterator instead of borrowing them.
     - map to a closure which parses each &str to a i64 option and then unwraps it.
     ```
     .map(|n| str::parse::<i64>(&n).unwrap())
     ```
1. Documentation was a bit difficult to get a handle on
   - I haven't used Markdown before, but I figured it out quickly.
   - However, getting the crate documentation to look good was a bit tricky.  Since there aren't any public functions, there isn't really much documentation that gets included.

### Validation(How I Tested It)
I used TDD(Test Driven Development) for this small program.  It is much easier than the TDD I did in the Adv Java Programming class I took last year.  Being able to include the tests inline with the code makes it very simple.  As I stated above, I created the function prototypes(with dummy bodies with returned 0) and then added the tests that I felt would represent the function well.
For each function, I created a set of tests which include valid and invalid inputs - each with a range of inputs.  I didn't include any tests for the main function as there is little that could go wrong here, so I focused on unit tests for the calc functions.

I also included tests which checked for invalid inputs such as 0 for gcd or lcm or no values for any of the functions. A selection of the tests are included below:

```
///////////////////////
//
// Greatest Common Divisor
//
#[test]
fn test_gcd() {
    assert_eq!(gcd(vec![]), 0);
    assert_eq!(gcd(vec![12, 8]), 4);
    assert_eq!(gcd(vec![-12, -8]), 4);
    assert_eq!(gcd(vec![24, 12, 8]), 4);
    assert_eq!(gcd(vec![9, 12, 27, 63]), 3);
    assert_eq!(gcd(vec![-4, 12, 8]), 4);
    assert_eq!(gcd(vec![7, 13, 23, 57]),1);
}
#[test]
#[should_panic]
fn test_gcd_panic1() {
    gcd(vec![0, 10]);
}

#[test]
#[should_panic]
fn test_gcd_panic2() {
    gcd(vec![-20, 0, 10]);
}
```
