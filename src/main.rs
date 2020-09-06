use termion::{color, style};

fn main() {
    println!("{magenta}Magenta{blue}blue{green}Green{red}{bold}communism{reset}",
            bold  = style::Bold,
            magenta = color::Fg(color::Magenta),
            red   = color::Fg(color::Red),
            blue  = color::Fg(color::Blue),
            green = color::Fg(color::Green),
            reset = style::Reset);
}
