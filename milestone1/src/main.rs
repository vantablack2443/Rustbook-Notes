
// lecture video 12: concepts of lifetime, traits, and generics in Rust

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str, 
    ann: T
) -> &'a str 
where
    T: Display,  // this is a trait bound, it specifies that T must implement the Display trait
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(string1.as_str(), string2, "Comparing two strings");
    println!("The longest string is {}", result);
}
