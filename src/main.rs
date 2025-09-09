use std::{env, io, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <text> [--width N] [--delay MS] [--padding N ]", args[0]);
        return;
    }

    let mut width: usize = 20;  // default width
    let mut delay: u64 = 150;   // default delay in milliseconds
    let mut padding: usize = 5;   // default padding between loops
    let mut text_args: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--width" | "-w" if i + 1 < args.len() => {
                width = args[i + 1].parse().unwrap_or(width);
                i += 2;
            }
            "--delay" | "-d" if i + 1 < args.len() => {
                delay = args[i + 1].parse().unwrap_or(delay);
                i += 2;
            }
            "--padding" | "-p" if i + 1 < args.len() => {
                padding = args[i + 1].parse().unwrap_or(padding);
                i += 2;
            }
            _ => {
                text_args.push(args[i].clone());
                i += 1;
            }
        }
    }
    
    let chars: Vec<char> = (text_args.join(" ") + &*' '.to_string().repeat(padding))
        .chars().collect();

    let mut pos = 0;
    loop {
        let mut view = String::new();
        for i in 0..width {
            let idx = (pos + i) % chars.len();
            view.push(chars[idx]);
        }

        print!("\r{view}");
        io::Write::flush(&mut io::stdout()).unwrap();

        thread::sleep(Duration::from_millis(delay));
        pos = (pos + 1) % chars.len();
    }
}
