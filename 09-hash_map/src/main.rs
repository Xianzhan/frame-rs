use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("k".to_string(), 107);
    println!("{:?}", m);

    m.entry("k".to_string()).or_insert(100);
    println!("{:?}", m);

    m.insert("l".to_string(), 108);
    for ele in &m {
        println!("k:{},v:{}", ele.0, ele.1);
    }
    println!("{:?}", m);

    m.iter().for_each(|(k, v)| {
        println!("for_each: {}={}", k, v);
    });

    let v = &m["k"];
    println!("v: {}", v);

    let v = m.get("k");
    if let Some(v) = v {
        println!("Some({})", v);
    }

    // zip
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
}
