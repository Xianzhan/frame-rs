enum Number {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
}

fn main() {
    let mut num = Number::Byte(10);
    num = Number::Int(1);
    num = Number::Long(2);

    match num {
        Number::Byte(b) => println!("byte: {}", b),
        Number::Short(s) => println!("short: {}", s),
        Number::Int(i) => println!("int: {}", i),
        Number::Long(l) => println!("long: {}", l),
        // _ => println!("impossible"),
    }

    if let Number::Short(s) = Number::Short(11) {
        println!("short: {}", s);
    }
}
