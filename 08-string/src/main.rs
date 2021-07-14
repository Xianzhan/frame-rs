fn main() {
    // empty
    let mut s = String::new();
    s.push_str("push_str");
    println!("{}", s);

    let mut s = String::from("from");
    println!("{}", s);
    s.push_str(" push_str");
    println!("{}", s);

    let mut s = "&str".to_string();
    println!("{}", s);
    s.push('0');
    println!("{}", s);

    // +
    let s = "a ".to_string() + "+ b";
    println!("{}", s);

    // format!
    let s = format!("{} + {}", "format", "!");
    println!("{}", s);

    // foreach
    let s = "Здравствуйте";
    for c in s.chars() {
        println!("c: {}", c);
    }
    for b in s.bytes() {
        println!("b: {}", b);
    }
    println!("{}", s);
}
