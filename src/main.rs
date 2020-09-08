use ansi_term::{Colour};
use structopt::StructOpt;

#[derive(StructOpt)]
enum MsgColour {
    Blue,
    Yellow,
}

fn parse_color(src: &str) -> Result<MsgColour, String> {
    if src == "blue" {
        Result::Ok(MsgColour::Blue)
    } else if src == "yellow" {
        Result::Ok(MsgColour::Yellow)
    } else {
        Result::Err("Bad color!".to_string())
    }
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(
        short, long,
        parse(try_from_str = parse_color),
        default_value = "blue",
    )]
    colour: MsgColour
}

fn main() {
    let opt = Opt::from_args();

    let colour = match opt.colour {
        MsgColour::Blue => Colour::Blue,
        MsgColour::Yellow => Colour::Yellow,
    };

    println!(
        "{}, {}!",
        Colour::Blue.bold().paint("Hello"),
        colour.underline().paint("World")
    );
}
