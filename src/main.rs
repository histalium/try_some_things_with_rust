use std::io;
use std::io::Write;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    test_ansi();
    play_minesweeper();
}

fn test_ansi() {
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

fn play_minesweeper() {
    let size = 4;
    let number = [1, 1, 1, 0, 2, -1, 2, 0, 2, -1, 3, 1, 1, 1, 2, -1];
    let mut vis = [false; 16];
    print_field(number, vis);

    while !find_all(number, vis) {
        print!("enter the coordinates: ");
        io::stdout().flush().unwrap();
        let game_over: bool = match read_line() {
            Some(v) => {
                let mut game_over = false;
                let mut chars = v.chars();
                let expected: usize = 3;
                if chars.clone().count() != expected {
                    println!("wrong input");
                    print!("enter the coordinates: ");
                } else {
                    let c1 = chars.next().unwrap() as i32;
                    let c2 = chars.next().unwrap() as i32;
                    let c = c1 - 97;
                    let r = c2 - 48;
                    if c < 0 || c > size {
                        println!("wrong input");
                        print!("enter the coordinates: ");
                    } else if r < 0 || r > size {
                        println!("wrong input");
                        print!("enter the coordinates: ");
                    } else {
                        let cell = r * size + c;
                        let b = number[cell as usize];
                        vis[cell as usize] = true;
                        print_field(number, vis);
                        if b == -1 {
                            println!("you lose");
                            game_over = true;
                        }
                    }
                }

                game_over
            }
            None => false,
        };

        if game_over {
            return;
        }
    }
    println!("you won");
}

fn find_all(numbers: [i32; 16], vis: [bool; 16]) -> bool {
    let size: usize = 16;
    for i in 0..size {
        let n = numbers[i];
        let v = vis[i];
        if n != -1 && v == false {
            return false;
        }
    }

    return true;
}

fn print_field(numbers: [i32; 16], vis: [bool; 16]) {
    let size = 4;
    print!("  ");
    for n in 0..size {
        let v = get_row_text(n);
        print!("   {}", v);
    }
    println!();
    print!("   -");
    for _ in 0..size {
        print!("----");
    }
    println!();

    for n in 0..size {
        print!("{}  |", n);
        for m in 0..size {
            let i = (n * 4 + m) as usize;
            let s = numbers[i];
            if vis[i] == false {
                print!(" . |");
            } else if s == -1 {
                print!(" * |");
            } else {
                print!(" {} |", s);
            }
        }
        println!();
        print!("   -");
        for _ in 0..size {
            print!("----");
        }
        println!();
    }
}

fn read_line() -> Option<String> {
    let stdin = io::stdin();

    let mut input = String::new();
    match stdin.read_line(&mut input) {
        Ok(_) => Some(input),
        Err(_) => None,
    }
}

fn get_row_text(v: i32) -> String {
    let utf = (v + 97) as u16;
    let s = match String::from_utf16(&[utf]) {
        Ok(ss) => ss,
        Err(_) => String::from(" "),
    };
    return s;
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
