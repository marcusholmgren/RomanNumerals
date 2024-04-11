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
            println!("{}", to_arabic(input))
        }
        _ => println!("Please try again with the --help option"),
    }
    println!("Hello from roman: MCMXCIV");
    //let number = to_arabic(String::from("MCMXCIV"));
    /*
    println!("to arabic: {}", number);

    println!("Year 2023: {}", to_roman(2023));
    println!("Year 2024: {}", to_roman(2024));
    println!("Born 1974: {}", to_roman(1974));

     */
    //println!("Year 2024: {}", to_roman(2024));
}
