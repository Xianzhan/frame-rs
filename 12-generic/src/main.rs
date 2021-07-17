// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// impl 后面没有泛型，表示该代码块实现特定的类型方法
impl Point<String, i32> {
    fn x_y(&self) {
        println!("String: {}, i32: {}", &self.x, &self.y);
    }
}

fn main() {
    let int = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}\n{:?}", int, float);

    let f_i = Point { x: 1.0, y: 314 };
    println!("x: {}", f_i.x());
    let i_s = Point {
        x: 6,
        y: "str".to_string(),
    };
    let f_s = f_i.mixup(i_s);
    println!("{:?}", f_s);

    let s_i = Point {
        x: "hello".to_string(),
        y: 32,
    };
    s_i.x_y();
}
