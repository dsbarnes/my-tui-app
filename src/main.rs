use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

fn main() {
    // Enter raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Write to stdout (note that we don't use `println!`)
    writeln!(stdout, "Hey there.").unwrap();
    writeln!(stdout, "Something else!!").unwrap();
    writeln!(stdout, "Why does writeln! do this?").unwrap();
    // Here the destructor is automatically called, and the terminal state is restored.
}
