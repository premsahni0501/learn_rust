use std::cmp::PartialOrd;
fn main() {
    let numbers = vec![1, 3, 4, 2, 6, 8, 0];
    let chars = vec!['a', 'v', 'd', 'y', 'o'];
    let l_char = largest(&chars);
    let l_num = largest(&numbers);
    println!("Hello, world! {l_char} {l_num}");

    let p = Point { x: 5, y: 10 };
    println!("Point: {}, {}", p.x, p.y);

    let esome = Option::Some(34);
    // let enone = Option::None;
    println!("some: {:#?}", esome);
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

struct Point<T> {
    x: T,
    y: T,
}
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
