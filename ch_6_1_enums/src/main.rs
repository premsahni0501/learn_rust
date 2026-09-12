#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V4Int(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddrV4 {
    address: (u8, u8, u8, u8),
}
#[derive(Debug)]
struct IpAddrV6 {
    address: String,
}

#[derive(Debug)]
enum IpAddrStruct {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct QuitMessage;

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct WriteMessage(String);
#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32);

#[derive(Debug)]
enum Message2 {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}
impl Message {
    fn call(&self) {
        println!("{self:#?}");
    }
}
impl Message2 {
    fn call(&self) {
        println!("{self:#?}");
    }
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    println!("Hello, world! {v4:#?}, {v6:#?}");

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let away = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home: {home:#?}, Away: {away:#?}");

    let v4_addr = IpAddrEnum::V4(String::from("127.0.0.1"));
    let v4_int_addr = IpAddrEnum::V4Int(127, 0, 0, 1);
    let v6_addr = IpAddrEnum::V6(String::from("::1"));
    println!("Enum V4: {v4_addr:#?}, V6: {v6_addr:#?}, V4Int: {v4_int_addr:#?}");

    let v4_struct = IpAddrStruct::V4(IpAddrV4 {
        address: (127, 0, 0, 1),
    });
    let v6_struct = IpAddrStruct::V6(IpAddrV6 {
        address: String::from("::1"),
    });
    println!("V4: {v4_struct:#?}, V6: {v6_struct:#?}");

    let m1 = Message::Write(String::from("Hello"));
    m1.call();
    let m2 = Message::Quit;
    m2.call();
    let m3 = Message::Move { x: 123, y: 342 };
    m3.call();
    let m4 = Message::ChangeColor(123, 231, 321);
    m4.call();

    let m1 = Message2::Write(WriteMessage(String::from("Hello2")));
    m1.call();
    let m2 = Message2::Quit(QuitMessage);
    m2.call();
    let m3 = Message2::Move(MoveMessage { x: 231, y: 321 });
    m3.call();
    let m4 = Message2::ChangeColor(ChangeColorMessage(123, 231, 321));
    m4.call();

    let some_char = Some('a');
    let some_num: Option<i32> = Some(5);

    let absent_num: Option<i32> = None;
    // let sum = 5 + absent_num;

    println!("char: {some_char:#?}, some_num: {some_num:#?}, absent_num: {absent_num:?}");
}
