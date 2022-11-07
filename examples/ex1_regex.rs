// ex1_regex, see https://docs.rs/regex/1.5.4/regex/

use regex::Regex;

fn check(regexp: &str, strs: &[&str]) {
    println!("Regex {:?}", regexp);
    let re = Regex::new(regexp).unwrap();
    for s in strs.iter() {
        println!("{:?} {:?}", s, re.is_match(s));
    }
    println!();
}

fn main() {
    let strs = &["ab", "abc", "abd", "abbc", "1c", "  abcd   "];

    // Accept "abc"
    check(r"abc", strs);

    // Accept "ab" followed by either "c" or "d", | is the "or"
    check(r"ab[c|d]", strs);

    // Accept "a" followed by one or more "b" followed by "c"
    check(r"a(b?)c]", strs);

    let strs = &["1234", " 1234", "12", ""];
    // ^ and t$ are anchors to the beginning and end
    // \d matches any digit, (\d)*, zero or more digits
    check(r"^(\d)+$", strs);

    // Accepts any number not containing digit 3
    check(r"^[0-9--3]+$", strs);

    // For now we have only checked acceptance.
    // Usually we want to get the captured content

    // The unmatched parts of the input string does not occur in the captures
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
        println!("Each complete capture: {}", &cap[0]);
    }

    // Make sure you fully understand what's going on here.
    // Check the docs at https://docs.rs/regex/1.5.4/regex/
}

// The Rust `regex` crate provides an efficient and flexible API
// for parsing of regular expressions. Readability, hmm, you judge.
//
// For the examples below you should construct the correct regexp
// to pass the tests.
// > cargo test --example ex1_regex
//
// As all tests fail from the beginning, focus at one test at the time, e.g.
// > cargo test --example ex1_regex test1
//
// You ran also just click on the "Run Test" button for each specific
// test in vscode.

#[test]
fn test1() {
    // Change the regexp to pass the tests
    // using the | operator
    let re = Regex::new(r"abc|def").unwrap();
    assert_eq!(re.is_match("abc"), true);
    assert_eq!(re.is_match("def"), true);
    assert_eq!(re.is_match("a"), false);
}

#[test]
fn test2() {
    // Change the regexp to pass the tests
    // using the ? operator
    let re = Regex::new(r"a?bc").unwrap();
    assert_eq!(re.is_match("abc"), true);
    assert_eq!(re.is_match("bc"), true);
    assert_eq!(re.is_match("ac"), false);
}

#[test]
fn test3() {
    // Change the regexp to pass the tests
    // using the * operator
    let re = Regex::new(r"abc*").unwrap();
    assert_eq!(re.is_match("abccc"), true);
    assert_eq!(re.is_match("ab"), true);
    assert_eq!(re.is_match("ac"), false);
}

#[test]
fn test4() {
    // Change the regexp to pass the tests
    // using the anchor operators
    let re = Regex::new(r"^ab$").unwrap();
    assert_eq!(re.is_match("ab"), true);
    assert_eq!(re.is_match(" ab"), false);
    assert_eq!(re.is_match("ab "), false);
    assert_eq!(re.is_match("ac"), false);
}

#[test]
fn test5() {
    // Change the regexp to pass the tests
    // using direct subtraction and *
    // HINT: you also need something else here
    // to make the last assertion fail
    let re = Regex::new(r"^[0-9--3]*$").unwrap();
    assert_eq!(re.is_match("12"), true);
    assert_eq!(re.is_match("245"), true);
    assert_eq!(re.is_match("456789"), true);
    assert_eq!(re.is_match("23"), false);
}

#[test]
fn test6() {
    // This one is a little tricky.
    // We want to reject any month larger than 12

    let re = Regex::new(r"^\d{4}-(0[1-9]|1[0-2])-\d{2}$").unwrap();

    assert_eq!(re.is_match("2012-03-14"), true);
    assert_eq!(re.is_match("2012-12-31"), true);
    assert_eq!(re.is_match("2012-13-31"), false);
    assert_eq!(re.is_match("2012-20-31"), false);
}

#[test]
fn test7() {
    // We want to reject any illegal date taking leap years (skottår)
    // into account.
    //
    // Why is it fundamentally unsuitable to use regexp
    // to solve this problem?
    //
    // [YOUR ANSWER HERE:]
    //
    // let re = Regex::new(r"").unwrap();

    // assert_eq!(re.is_match("2024-02-29"), true);
    // assert_eq!(re.is_match("2028-02-29"), true);
    // assert_eq!(re.is_match("2022-02-29"), false);
}
