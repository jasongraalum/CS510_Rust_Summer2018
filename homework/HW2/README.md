# CS510 Rust Programming
# Homework #2

## Simple Calculator Program
## Student: Jason Graalum
## Date: July 21, 2018

## Lessons Learned
### bob
The bob program was straight-forward.  I created two booleans using a functional programming style. The first, yell, indicated whether the input string was all caps, in which case the teenager was being yelled at. The second, question, indicated whether the last character was a question mark, in which case the teenager is being asked a question.
The remainder of the program is a simple if-else block looking checking the different combinations of yell and question. (The first if statement looks for a empty message, which indicates that there was silence.)

The one trick was correctly checking for all capitals. You can't look for absense of lower case, as there may be a message with no alpha characters. Second, you can't look for all upper case as again there may be non-alpha characters. So, yell is a combination of no lower case alphas and some upper case alphas.

### isbn-verifier
The isbn-verifier was a little trickier - due to some special cases. Again, I used a functional programming style to solve this problem. 

The first step is to remove all '-' characters. Next, we create a "char indices" list which include tuples of each character along with the position of the character in the input str. Then we use a series of `if-else's` fed by a `for_each` on the `char_indices` to check for the different cases.  As first, I was confused about the use of 'X' - the insrtuctions were a bit unclear. But, by running the test cases, I found that an 'X' can in any position in the isbn code.

One trick I used was, while summing up the isbn code, if I encountered an illegal code or character, I set the `sum `to -1.  This is essential a dead-end code. If the sum ever becomes -1, it will stay at that value until the end of the check.  And since `-1 % 11` is never 0, the function will return `false`.


### pythagorean-triplet
For this problem, I was a bit lazy and wrote a set of simple 'for' loops. But, given that we are only calcuting the triplet for a single sum, it seemed reasonable. I may take another look to see if function programing style could be used.

### raindrops
Again, this problem was very straight forward and I used a simple set of `if-else` statements to check for the value of each `mod` option.

### robot simulator
This problem required the most code of the five. Given the code-stub with the Direction `enum` and Robot `struct` I created the `impl` for the Robot with the six functions: 

* `new() -> Self` 
* `turn_right(self) -> Self`
* `turn_left(self) -> Self` 
* `advance(self) -> Self`
* `instructions(self, instruction: &str) -> Self`
* `position(&self) -> (isize, isize)`
* `direction(&self) -> &Direction`

The first function is a simple constructor that returns ownership of a new robot with a given location and direction. The next 3 functions comsume the robot and produce a new robot with an updated location or direction.

The interesting function is the instructions function. This is a **recursive** function that comsumes the first letter of the instructions. It then creates and returns a new robot by calling one of the three modification functions, `turn_right`, `turn_left`, or `advance` followed by the instructions command(this is the recursion) with the remaining instructions str. Here is the `match` code:

```
        match instructions.split_at(1) {
            ("R", i) => self.turn_right().instructions(i),
            ("L", i) => self.turn_left().instructions(i),
            ("A", i) => self.advance().instructions(i),
            _ => self,
        }
```

Eventually, the robot is called with an empty instruction string, at which point it simply returns itself.



