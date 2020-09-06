use termion::color;
use termion::clear;

fn main() {
    println!("{red}Hello, world!{reset}",
        red     = color::Fg(color::Red),
        reset   = color::Fg(color::Reset)
    );

    /*
     * clear::All
     * clear::CurrentLine
     * clear::AfterCursor
     * clear::BeforeCursor
     * See the docs: https://docs.rs/termion/1.5.5/termion/
     */
    println!("{}", clear::All);
}
