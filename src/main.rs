use ansi_term::Colour::{Blue, Yellow};

fn main() {
    println!(
        "{}, {}!",
        Blue.bold().paint("Hello"),
        Yellow.underline().paint("World")
    );
}
