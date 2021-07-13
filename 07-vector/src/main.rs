fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    println!("v: {:?}", v);

    let v2 = vec![3, 4, 5];
    println!("v: {:?}", v2);

    let one = v2[1];
    println!("one: {}", one);
    if let Some(n) = v2.get(1) {
        println!("Some({})", n);
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10;
    }
    println!("v: {:?}", v);

    // 使用枚举存储不同类型
    enum Number {
        Byte(i8),
        Short(i16),
        Int(i32),
        Long(i64),
    }
    let ev = vec![
        Number::Byte(1),
        Number::Short(2),
        Number::Int(3),
        Number::Long(4),
    ];
    for num in &ev {
        match num {
            &Number::Byte(b) => println!("byte: {}", b),
            &Number::Short(s) => println!("short: {}", s),
            &Number::Int(i) => println!("int: {}", i),
            &Number::Long(l) => println!("long: {}", l),
        }
    }
}
