use termion::color;

fn main() {
    println!("{red}Hello, world!{reset}",
        red     = color::Fg(color::Red),
        reset   = color::Fg(color::Reset)
    );
}
