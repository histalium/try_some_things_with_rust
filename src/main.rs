use std::io;
use std::io::Read;
use std::io::Write;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    go_to_screen();
    set_colors();
    write_out(b"\x1B[3;7H@"); // put @ at row 3 column 7

    let res = func_test(5, &times2);
    let times3 = times(3);
    let v9 = times3(3);
    let res2 = func_test(4, &times3);

    println!("Hello, world! {} {} {}", res, v9, res2);

    let size = terminal_size();

    match size {
        Some((Width(w), Height(h))) => println!("w{} h{}", w, h),
        _ => {}
    };

    let stdin = io::stdin();

    let mut input = String::new();
    match stdin.read_line(&mut input) {
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
    return_screen();
}

fn print_user(user: &User) {
    println!("{}", user.username);
    println!("{}", user.email);
}

fn go_to_screen() {
    write_out(b"\x1B[?1049h");
}

fn set_colors() {
    write_out(b"\x1B[34;43m");
    write_out(b"\x1B[2J");
}

fn write_out(buf: &[u8]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match handle.write_all(buf) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }

    handle.flush().expect("Error setting Ctrl-C handler");
}

fn return_screen() {
    println!("bye");
    write_out(b"\x1B[?1049l");
}

struct User {
    username: String,
    email: String,
}

fn func_test(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
    let x = f(value);
    return x;
}

fn times2(value: i32) -> i32 {
    2 * value
}

fn times(value: i32) -> impl Fn(i32) -> i32 {
    return move |v| {
        return value * v;
    };
}
