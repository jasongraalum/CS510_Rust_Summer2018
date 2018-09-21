# CS510 Rust Programming
# Homework #3

## Student: Jason Graalum
## Date: July 28, 2018

## Lessons Learned
### crypto-square 
This program took me a while to get write. Manipulating vectors and Strings is not straight forward. In my first attempt, I tried to keep everything in a functional style. But, there seemed to be too many cases and dependencies - such as pulling out the row and columns sizes before creating the output. Also, trying to create the String directly instead of not easy.

### protein-translation
This was very straight forward. I did have to look up using a static String to compare for the "stop codon" result. Other than that, this was very straight forward. I used a three-deep HashMap to break down the three-character codons.  They searching was as easy as going through the different HashMap levels.

### robot-names 
This is a very simple program - if uniqueness is not guaranteed. All that is needed are three calls to a random function and a couple of "getter/setter" functions.

### sieve
For the sieve, I used a separate HashSet to track the numbers which are not primes.  I then incremented a "next" value to the next value not in the HashMap. This value was added to the output Vector and an iterator was used to fill in the HashMap with subsequence multiples.

### wordy
For this program, I created a "state-machine" with in a fold iterator method. This combined the various values and operators returning a result(or an error.)
The state machine has 7 states with an Error dead-end state as show below:
```
enum Op {
    Start,
    Plus,
    Minus,
    Multiplied,
    Divided,
    Next,
    Error,
}
```

The fold operation uses an accumulator(acc) and current state(curr_op). The Start state initializes the accumulator with the first value. The Next state indicates that another operation is expected.
Each state can move into the dead-end Error state if the next value of the input iterator is not what is expected.


```
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
```
