use termion::{color, clear, cursor};

// Work out how that grid works

fn main() {
    println!("{clear}{goto}{red}more red than any comrade{reset}",
         // Full screen clear.
         clear = clear::All,
         // Goto the cell.
         goto  = cursor::Goto(4, 2),
         red   = color::Fg(color::Red),
         reset = color::Fg(color::Reset));


    println!("{goto}{red}more red than any comrade{reset}",
             goto  = cursor::Goto(6, 6),
             red   = color::Fg(color::Red),
             reset = color::Fg(color::Reset));


    // Clear the line and print some new stuff
    println!("Otherwise!{goto}{magenta}magenta{blue}blue{green}green{red}communism{reset}",
            goto    = cursor::Goto(8, 3),
            magenta = color::Fg(color::Magenta),
            red     = color::Fg(color::Red),
            blue    = color::Fg(color::Blue),
            green   = color::Fg(color::Green),
            reset   = color::Fg(color::Reset));
}
