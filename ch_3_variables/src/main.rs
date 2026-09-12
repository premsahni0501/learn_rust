use std::io;

fn main() {
    // variables_example();
    // int_float_example();
    // tuple_example();
    // array_example();
    // condition_example();
    // loop_example();
    // disambiguating_loop();
    // while_loop();
    // array_loop();
    // for_loop();
    // temperature_converter();
    // fibonacci_series();
    // christmas_carol();
}

fn christmas_carol() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];

    // let mut lyrics = String::new();
    let days_count = days.len();
    let gifts_count = gifts.len();
    for i in 0..days_count {
        println!(
            "[Verse {0}]\nOn the {1} day of Christmas, my true love sent to me",
            i + 1,
            days[i]
        );
        for j in (gifts_count - i - 1)..gifts_count {
            println!("{0}", gifts[j]);
        }
        println!("\n");
    }
}

fn fibonacci_series() {
    let mut count_till = String::new();

    println!("Please enter the number to generate fibonacci series");

    io::stdin()
        .read_line(&mut count_till)
        .expect("Failed to read line");

    let count_till = count_till
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let mut a = 0;
    let mut b = 1;
    let mut total = 0;
    loop {
        if total == 0 {
            print!("{a}, {b}");
        }
        total = a + b;
        print!(", {total}");
        if b + total > count_till {
            break;
        }
        a = b;
        b = total;
    }
}

fn temperature_converter() {
    let mut temperature = String::new();

    println!("Enter temperature: eg. 36");
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: u32 = temperature
        .trim()
        .parse()
        .expect("Please enter valid temperature number");

    println!("Enter type: c for celcius or f for farhenheit");
    let mut temp_type = String::new();

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line");

    let temp_type = temp_type.trim();

    let mut temp = 0;
    let mut other_type = "f";
    if temp_type == "c" {
        temp = convert_c_to_f(temperature);
    } else {
        temp = convert_f_to_c(temperature);
        other_type = "c";
    }

    println!("{temperature}{temp_type} is {temp}{other_type}");
}

fn convert_c_to_f(temp: u32) -> u32 {
    (temp * 9 / 5) + 32
}

fn convert_f_to_c(temp: u32) -> u32 {
    (temp - 32) * 5 / 9
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5, 6];
    for element in a {
        println!("{element}")
    }

    for number in (1..4).rev() {
        println!("Counting down {number}");
    }
    println!("Lift off!");
}

fn array_loop() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut i = 0;
    while i < a.len() {
        println!("Element at index {i} is {0}", a[i]);
        i += 1;
    }
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("Count down: {number}");
        number -= 1;
    }
    println!("Ignition!!!!");
}

fn disambiguating_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count: {count}");

        let mut remaining = 10;

        'inner: loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break 'inner;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count: {count}");
}

fn loop_example() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn condition_example() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn variables_example() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let _spaces = spaces.len();

    let spaces = "   ";
    let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number");

    println!("{spaces}, {guess}");
}

fn int_float_example() {
    let i: u16 = 120;
    let j: u16 = 210;
    println!("{0}", i + j);
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");
}
fn tuple_example() {
    let tup = (500, 6.4, 1);

    let (x, y, _z) = tup;

    println!("The value of y is: {y} {0} {1} {2}", x, tup.1, tup.2);
}
fn array_example() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");
    let index = read_and_parse_line();
    if index < a.len() {
        let element = a[index];

        println!("The value of the element at index {index} is {element}");
    } else {
        println!("You entered a larger/invalid index value");
    }
}

fn read_and_parse_line() -> usize {
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please enter valid index number");
    index
}
