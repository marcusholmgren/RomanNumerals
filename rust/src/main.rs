use clap::{Parser, Subcommand};
use roman_numerals::{to_arabic, to_roman};

#[derive(Debug, Parser)]
#[command(author, version, about = "Roman numerals converter", long_about = None)]
struct Cli {
    /// Conversion argument
    #[command(subcommand)]
    //#[arg(required = true)]
    convert: Option<Conversion>,
}

/// Conversion argument values
#[derive(Debug, Subcommand)]
enum Conversion {
    /// Convert arabic numbers into roman numbers.
    #[command(short_flag_alias = 'a')]
    Arabic { input: u32 },
    /// Convert roman numbers into arabic numbers.
    #[command(short_flag_alias = 'r')]
    Roman { input: String },
}

fn main() {
    let matches = Cli::parse();
    match matches {
        Cli {
            convert: Some(Conversion::Arabic { input }),
        } => {
            println!("{}", to_roman(input))
        }
        Cli {
            convert: Some(Conversion::Roman { input }),
        } => {
            match to_arabic(input) {
                Ok(value) => println!("{}", value),
                Err(message) => eprintln!("Error: {}", message),
            }
        }
        _ => println!("Please try again with the --help option"),
    }
}
