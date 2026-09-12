// use aggregator::{NewsArticle, SocialPost, Summary};

use ch_10_2_traits::{NewsArticle, SocialPost, Summary, notify};

fn main() {
    let post = SocialPost {
        username: String::from("Prem"),
        content: String::from("Hi"),
        reply: false,
        repost: true,
    };
    println!("1 new post {}", post.generate());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.generate());

    notify(&article);

    let s1 = String::from("prem");
    let s2 = "chand";
    let l = longest(s1.as_str(), s2);
    println!("Longest: {l}");

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
    // longest2(s1.as_str(), s2);
    let w = first_word(&s1);
    println!("First: {w}")
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// fn longest2<'a>(x: &str, y: &str) -> &'a str {
// let result = String::from("really long string");
// result.as_str()
// }
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
