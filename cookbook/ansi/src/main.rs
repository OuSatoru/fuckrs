use ansi_term::{Colour, Style};

fn main() {
    println!("this is {} in color, {} in color, and {} in color. {}. {}.",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"),
        Style::new().bold().paint("This is bold"),
        Colour::Yellow.bold().paint("yellow and bold"));
}
