use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let file_path = "./src/hello.txt";
    let file_result = File::open(file_path);
    let file_content = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {e}"),
            },
            _ => {
                panic!("Problem opening the file: {error}")
            }
        },
    };
    println!("File Content: {:?}", file_content.metadata());

    let greeting_file = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_path).unwrap_or_else(|e| {
                panic!("Problem creating the file: {e:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}")
        }
    });
    println!("File Content: {:?}", greeting_file.metadata());

    let file_res = File::open(file_path).unwrap();
    println!("File Content: {:?}", file_res.metadata());

    let username = read_user_from_file(".gitignores").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            println!("File not found: {e}");
            return String::new();
        } else {
            println!("Unable to open file");
            return String::new();
        }
    });

    println!("Username: {username}");

    let content = read_file_content("file_path").unwrap_or_else(|e| {
        println!("Error: {e}");
        return String::new();
    });
    println!("Content: {content}");
    let c = read_file_content_short(file_path).unwrap_or_else(|e| {
        println!("Error: {e}");
        return String::new();
    });
    println!("Content: {c}");
}

fn read_user_from_file(file_path: &str) -> Result<String, io::Error> {
    let file_res = File::open(file_path);
    let mut username_file = match file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e),
    };
}

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut file_result = File::open(file_path)?;
    let mut file_content = String::new();
    file_result.read_to_string(&mut file_content)?;
    return Ok(file_content);
}

fn read_file_content_short(file_path: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();
    File::open(file_path)?.read_to_string(&mut file_content)?;
    return Ok(file_content);
}
