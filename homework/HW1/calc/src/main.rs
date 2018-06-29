//
//!
//!# calc: Simple Calculator Program
//!
//!## Implemented functions:
//!*  sum
//!*  product
//!*  gcd
//!*  lcm
//!
//!## Usage:
//!*    calc sum value1 value2 ...
//!*    calc product value1 value2 ...
//!*    calc gcd value1 value2 ...
//!*    calc lcm value1 value2 ...
//!*    calc -h | --help
//!*    calc -v | --version
//!
//!## Options:
//!*    -h --help     Show this message
//!*    -v --version  Show version
//
fn main() {
    let mut cmd_args: Vec<String> = std::env::args().collect();

    // Pull off the program name and command/option argument(first)
    let _prog: String = cmd_args.remove(0);
    let cmd: String = cmd_args.remove(0);

    // Help and Version options
    match cmd.as_ref() {
        "-h" | "--help" => {
            print_usage();
            return;
        }
        "-v" | "--version" => {
            print_version();
            return;
        }
        _ => {}
    };

    // If no args
    assert!(cmd_args.len() > 0);

    // Parse args to i64 type
    let args: Vec<i64> = cmd_args
        .into_iter()
        .map(|n| str::parse::<i64>(&n).unwrap())
        .collect();

    // Calculation selection
    let result = match cmd.as_ref() {
        "gcd" => gcd(args),
        "product" => product(args),
        "sum" => sum(args),
        "lcm" => lcm(args),
        _ => {
            eprintln!("Invalid function: {}", &cmd);
            print_usage();
            return;
        }
    };

    //Output result to stdout
    println!("{}", result);
}

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
    assert_eq!(gcd(vec![7, 13, 23, 57]), 1);
}
#[test]
#[should_panic]
fn test_gcd_panic1() {
    gcd(vec![0, 10]);
}

#[test]
#[should_panic]
fn test_gcd_panic2() {
    gcd(vec![20, 0, 10]);
}

//
/// Return the Greatest Common Divisor(gcd) of values
//
fn gcd(mut vals: Vec<i64>) -> i64 {
    match vals.len() {
        0 => 0,
        1 => vals[0].abs(),
        2 => {
            let mut n: i64 = vals[0].abs();
            let mut m: i64 = vals[1].abs();
            assert!(n != 0 && m != 0);
            while m != 0 {
                if m < n {
                    let t = m;
                    m = n;
                    n = t;
                }
                m = m % n;
            }
            n
        }
        _ => {
            let n = vals.remove(0);
            gcd(vec![n, gcd(vals)])
        }
    }
}

//
// Include set of tests is no values, singeltons, pairs, and more
// Include negative numbers
// The sum function should never produce a panic
// It can take 0 or more arguments
//
#[test]
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

//
// Include set of tests is no values, singeltons, pairs, and more
// Include negative numbers
// The product function should never produce a panic
// It can take 0 or more arguments
//
//
#[test]
fn test_product() {
    assert_eq!(product(vec![]), 1);
    assert_eq!(product(vec![0]), 0);
    assert_eq!(product(vec![1]), 1);
    assert_eq!(product(vec![10]), 10);
    assert_eq!(product(vec![3, 10]), 30);
    assert_eq!(product(vec![1, 2, 3]), 6);
    assert_eq!(product(vec![1, 2, 3, 4]), 24);

    assert_eq!(product(vec![-10]), -10);
    assert_eq!(product(vec![-3, 10]), -30);
    assert_eq!(product(vec![-1, 2, -3]), 6);
    assert_eq!(product(vec![-1, -2, -3, -4]), 24);
}

//
// Return product of values
//
fn product(vals: Vec<i64>) -> i64 {
    vals.iter().fold(1, |n, i| n * i)
}

///////////////////////
//
// Least Common Multiple
//
//
// Include tests with no arguments, singeltons, pairs and more
// Include tests with negative numbers(result should be the same
// as with positive numbers)
// lcm will panic if gtiven a value of 0.
//
#[test]
fn test_lcm() {
    assert_eq!(lcm(vec![]), 0);
    assert_eq!(lcm(vec![1, 2, 3]), 6);
    assert_eq!(lcm(vec![2, 4]), 4);

    assert_eq!(lcm(vec![-1, 2, 3]), 6);
    assert_eq!(lcm(vec![-2, -4]), 4);
}
#[test]
#[should_panic]
fn test_lcm_panic1() {
    lcm(vec![0, 10]);
}
#[test]
#[should_panic]
fn test_lcm_panic2() {
    lcm(vec![-20, 0, 10]);
}

//
// Return Least Common Multiple of values
//
fn lcm(mut vals: Vec<i64>) -> i64 {
    match vals.len() {
        0 => 0,
        1 => vals[0],
        2 => {
            let mut n: i64 = vals[0];
            let mut m: i64 = vals[1];
            assert!(n != 0 && m != 0);
            (&n.abs() * &m.abs()) / gcd(vec![n, m])
        }
        _ => {
            let n = vals.remove(0);
            lcm(vec![n, lcm(vals)])
        }
    }
}

///
/// Display calc version
///
fn print_version() {
    eprintln!("calc: 1.0");
    eprintln!("");
}

///
/// Display calc usage message
///
fn print_usage() {
    eprintln!("Usage:");
    eprintln!("\tcalc sum value1 value2 ..");
    eprintln!("\tcalc product value1 value2 ..");
    eprintln!("\tcalc gcd value1 value2 ..");
    eprintln!("\tcalc lcm value1 value2 ..");
    eprintln!("\tcalc -h | --help");
    eprintln!("\tcalc -v | --version");
    eprintln!("");
    eprintln!("Options:");
    eprintln!("\t-h --help\tShow this message");
    eprintln!("\t-v --version\tShow version");
}
