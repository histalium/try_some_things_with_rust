use std::io;
use std::io::Write;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    println!("Hello, world!");

    let size = terminal_size();

    match size {
        Some((Width(w), Height(h))) => println!("w{} h{}", w, h),
        _ => {}
    };

    let mut input = String::new();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    //let bb = vec![27u8];
    // let ba = b"\x1B[31mout\n";
    //let ba = b"\x1B[38;5;0mout\n";
    //let concatenated: Vec<u8> = bb.iter().chain(ba).cloned().collect::<Vec<u8>>();

    let ba = b"\x1B[34;43m\x1B[2J\x1B[0;0H";
    match handle.write_all(ba) {
        Ok(_) => {
            println!("ok");
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    match io::stdin().read_line(&mut input) {
        Ok(b) => {
            println!("success 💩 {}, {}", b, input);
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let username = String::from("Hi");
    let user = User {
        username: username.clone(),
        email: String::from("me@example.com"),
    };

    // let us = user.email;

    println!("{}", username);

    print_user(&user);

    println!("{}", user.username);
    println!("{}", user.email);

    let x = ["Hi", "You"].join(" ");
    println!("{}", x);
}

fn print_user(user: &User) {
    println!("{}", user.username);
    println!("{}", user.email);
}

struct User {
    username: String,
    email: String,
}
