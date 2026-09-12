use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    let third = v[2];
    println!("third value: {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("third value: {value}"),
        None => println!("third value: no value"),
    };

    // let does_not_exist = &v[100];
    // println!("third value: no value {does_not_exist}");
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(value) => println!("third value: {value}"),
        None => println!("third value: no value"),
    };

    v.push(7);
    let first = &v[0];
    println!("First value: {first}");

    for i in v {
        println!("vector value {i}");
    }

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; // note s1 has been moved here and can no longer be used
    println!("{s1}, {s2}, {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1.clone() + "-" + &s2 + "-" + &s3;
    println!("{s1}, {s2}, {s3}");

    let mut s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    println!("{}", &s1[0..2]);
    s1 = String::from("tooin");

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for team {team_name} is {score}");

    for (key, value) in &scores {
        println!("Team {key} score is {value}");
    }
    println!("Scores {scores:?}");

    let field_name = String::from("fav_team");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("name: {field_name}, value: {field_value}");
    println!("{map:?}");
    let v = String::from("red");
    map.insert(&field_name, &v);
    println!("{map:?}");

    scores.entry(String::from("yellow")).or_insert(100);
    scores.entry(String::from("blue")).or_insert(101);
    println!("{scores:?}");

    let msg = "Pralay se kaun bachaye kai se bachaye";
    let mut msg_map = HashMap::new();

    for word in msg.split_whitespace() {
        let count = msg_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Message Map: {msg_map:?}");
}
