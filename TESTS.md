# Example tests

1. Given a list of digits, return the smallest number that could be formed from these digits, using the digits only once ( ignore duplicates).

```rust
fn min_value(mut digits: Vec<i32>) -> i32 {
    remove_duplicate_elements(&mut digits);
    let cummulative: String = digits.iter().map(ToString::to_string).collect();
    let cummulative: i32 = cummulative.parse().unwrap();
    cummulative
}

fn remove_duplicate_elements<T: Ord>(elements: &mut Vec<T>) {
    elements.sort();
    elements.dedup();
}

#[test]
fn basic_test() {
  assert_eq!(min_value(vec![1, 3, 1]), 13);
  assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
  assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}
```

2. Convert a regular english to letspeak

```rust
use std::collections::HashMap;

fn to_leet_speak(s: &str) -> String {
    let leets_map: HashMap<&str, &str> = [("A", "@"),
         ("B", "8"),
         ("C", "("),
         ("D", "D"),
         ("E", "3"),
         ("F", "F"),
         ("G", "6"),
         ("H", "#"),
         ("I", "!"),
         ("J", "J"),
         ("K", "K"),
         ("L", "1"),
         ("M", "M"),
         ("N", "N"),
         ("O", "0"),
         ("P", "P"),
         ("Q", "Q"),
         ("R", "R"),
         ("S", "$"),
         ("T", "7"),
         ("U", "U"),
         ("V", "V"),
         ("W", "W"),
         ("X", "X"),
         ("Y", "Y"),
         ("Z", "2"),
         (" ", " ")]
        .iter().cloned().collect();
    let mut result = "".to_string();
    for (_i, c) in s.chars().enumerate() {
        match leets_map.get(&*c.to_string()) {
            Some(&element) => result.push_str(&element),
            _ => result.push_str(" "), // c
        }
    }

    // s.chars().flat_map(|c| { map.get(&c) }).cloned().collect()

    String::from(result)
}

#[test]
fn leet() {
    assert_eq!(to_leet_speak("LEET"), "1337");
}

#[test]
fn codewars() {
    assert_eq!(to_leet_speak("CODEWARS"), "(0D3W@R$");
}

#[test]
fn hello_world() {
    assert_eq!(to_leet_speak("HELLO WORLD"), "#3110 W0R1D");
}

#[test]
fn lorem_ipsum() {
    assert_eq!(to_leet_speak("LOREM IPSUM DOLOR SIT AMET"), "10R3M !P$UM D010R $!7 @M37");
}

#[test]
fn quick_brown_fox() {
    assert_eq!(to_leet_speak("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"), "7#3 QU!(K 8R0WN F0X JUMP$ 0V3R 7#3 1@2Y D06");
}
```
