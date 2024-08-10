use std::env;

fn main() {
    let name = match env::var("NAME") {
        Ok(name) => name,
        Err(_) => match env::args().nth(1) {
            Some(arg) => arg,
            None => "world".to_string(),
        },
    };
    println!("Hello, {}!", name);
}
