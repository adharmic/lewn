use clap::{Parser, arg, command};
use lewn::{compute_check_digit, compute_checksum, compute_valid_sequence, validate_sequence};

#[derive(Parser, Default, Debug)]
#[command(author = "adharmic", version, about)]
/// An implementation of Luhn's checksum algorithm using Rust
pub struct Arguments {
    /// The number sequence to be operated upon (String)
    sequence: String,

    #[arg(short, long, action)]
    /// Determines if the provided sequence is a valid Luhn number
    validate: bool,

    #[arg(short, long, action)]
    /// Displays the calculated checksum for the provided sequence
    checksum: bool,

    #[arg(short, long, action)]
    /// Displays the valid Luhn number calculated from the provided sequence
    generate: bool,

    #[arg(short, long, action)]
    /// Displays the check digit calculated from the provided sequence
    digit: bool,
}

fn main() {
    let args = Arguments::parse();

    if (!args.validate && !args.checksum && !args.generate && !args.digit) || args.validate {
        let valid_sequence = validate_sequence(&args.sequence);
        let mut message = "is";
        if !valid_sequence {
            message = "is not";
        }
        println!("{} {} a valid Luhn number.", args.sequence, message)
    }

    if args.checksum {
        if validate_sequence(&args.sequence) {
            println!(
                "Generated checksum for {}: {}",
                args.sequence,
                compute_checksum(&args.sequence, false)
            );
        } else {
            println!(
                "Generated checksum for {}: {}",
                args.sequence,
                compute_checksum(&args.sequence, true)
            );
        }
    }

    if args.generate {
        println!(
            "Generated Luhn number from {}: {}",
            args.sequence,
            compute_valid_sequence(&args.sequence)
        );
    }

    if args.digit {
        println!(
            "Generated check digit for {}: {}",
            args.sequence,
            compute_check_digit(&args.sequence)
        );
    }
}
