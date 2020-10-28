# Try some things with rust

## Read from console

```rust
use std::io;

fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(b) => {
            println!("success 💩 {}, {}", b, input);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
```

## Write using stdout

```rust
use std::io;
use std::io::Write;

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let ba = b"Hi";
    match handle.write_all(ba) {
        Ok(_) => {
            println!("ok");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
```

## ANSI escape code

https://en.wikipedia.org/wiki/ANSI_escape_code

ESC = `\x1B`
CSI = `\x1B[`

```rust
let cs = b"\x1B[0;0H";   // clear screen
let co = b"\x1B[34;43m"; // 4bit color foreground blue background yellow
```

## Size console

toml file
```toml
[dependencies]
terminal_size = "0.1.13"
```

code file
```rust
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let size = terminal_size();

    match size {
        Some((Width(w), Height(h))) => println!("w{} h{}", w, h),
        _ => {}
    };
}
```