use std::{fmt::Display, str};

// The first rule: That each parameter that is a reference gets its own lifetime parameter.
// The second rule: If there is exactly one input lifetime parameter,
//                  that lifetime is assigned to all output lifetime parameters:
//                  fn foo<'a>(x: &'a i32) -> &'a i32.
// The third rule: If there are multiple input lifetime parameters,
//                 but one of them is &self or &mut self because this is a method,
//                 the lifetime of self is assigned to all output lifetime parameters.

fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let r1 = longest(s1.as_str(), s2);
    println!("The longest string is {}", r1);

    let r2 = longest_with_an_announcement("x1", "y23", "Display");
    println!("Display: {}", r2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
