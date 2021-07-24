extern crate add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );

    println!("and {} plus two is {}!", num, add_one::add_two(num));
}
