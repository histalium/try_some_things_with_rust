use rand::prelude::*;
use std::io;
use std::io::Write;

pub fn play_minesweeper() {
    let size = 4;
    let mut nums: Vec<i32> = (0..16).collect();
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
    let mines = [nums[0], nums[1], nums[2]];

    let mut number = [0; 16];

    for i in 0..16 {
        if mines.contains(&i) {
            number[i as usize] = -1;
        } else {
            number[i as usize] = number_of_mines(i, mines);
        }
    }

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

fn number_of_mines(i: i32, mines: [i32; 3]) -> i32 {
    let x = i % 4;
    let y = (i - x) / 4;
    let mut num = 0;
    let has_mine = is_mine(x - 1, y - 1, mines);
    if has_mine {
        num = num + 1;
    }
    let has_mine = is_mine(x, y - 1, mines);
    if has_mine {
        num = num + 1;
    }
    let has_mine = is_mine(x + 1, y - 1, mines);
    if has_mine {
        num = num + 1;
    }
    let has_mine = is_mine(x - 1, y, mines);
    if has_mine {
        num = num + 1;
    }
    let has_mine = is_mine(x + 1, y, mines);
    if has_mine {
        num = num + 1;
    }
    let has_mine = is_mine(x - 1, y + 1, mines);
    if has_mine {
        num = num + 1;
    }
    let has_mine = is_mine(x, y + 1, mines);
    if has_mine {
        num = num + 1;
    }
    let has_mine = is_mine(x + 1, y + 1, mines);
    if has_mine {
        num = num + 1;
    }

    return num;
}

fn is_mine(x: i32, y: i32, mines: [i32; 3]) -> bool {
    if x < 0 || x >= 4 {
        return false;
    }

    if y < 0 || y >= 4 {
        return false;
    }

    mines.contains(&(y * 4 + x))
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
