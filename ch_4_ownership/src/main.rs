fn main() {
    ownership1();
    ownership2();

    referencing();
    mut_ref();

    dang_ref();
}

fn dang_ref() {
    let s = dangle();
}
fn dangle() -> String {
    let s = String::from("Hello");
    return s;
}

fn mut_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{r1}, {r2}");
    let r3 = &mut s; // BIG PROBLEM
    println!("{r3}");
}

fn referencing() {
    let mut s = String::from("Hello");
    change(&mut s);
    let len = calculate_len(&s);
    println!("Length of {s} is {len}");
}

fn change(s: &mut String) {
    s.push_str(", World!");
}

fn calculate_len(s: &String) -> usize {
    return s.len();
}

fn ownership2() {
    let s1 = gives_ownership();
    let s2: String = String::from("hello");
    let s3 = takes_and_gives_back(&s2);
    println!("{s1}, {s2}, {s3}");
}

fn takes_and_gives_back(s: &String) -> String {
    return s.to_string();
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    return s;
}

fn ownership1() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
    println!("{x}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
