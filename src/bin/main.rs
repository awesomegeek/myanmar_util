use clap::{Parser, Subcommand};
use myanmar_util::{syllable_break, syllable_break_phoneme};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "myanmar-tools",
    author = "Naing Tun Win",
    version,
    about = "Tools for processing Myanmar text",
    long_about = "A collection of tools for processing Myanmar text including syllable breaking and other utilities."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Break Myanmar text into syllables
    #[command(
        about = "Break Myanmar text into syllables",
        long_about = "Breaks Myanmar text into syllables using linguistic rules and outputs the result with a specified separator."
    )]
    Syllablebreak {
        /// Separator to use between syllables
        #[arg(
            short,
            long,
            default_value = "|",
            help = "Character(s) to insert between syllables",
            long_help = "The character or characters to insert between syllables in the output. Default is '|'."
        )]
        separator: String,

        /// Input file path
        #[arg(
            short,
            long,
            required = true,
            help = "Path to the input file containing Myanmar text",
            long_help = "Path to the input file containing Myanmar text to be processed. This is required."
        )]
        input: Option<PathBuf>,

        /// Output file path
        #[arg(
            short,
            long,
            help = "Path to save the processed output (defaults to 'result.txt')",
            long_help = "Path where the processed text will be saved. If not specified, output will be written to 'result.txt' in the current directory."
        )]
        output: Option<PathBuf>,

        /// Break type (Phoneme or Morpheme)
        #[arg(
            short = 't',
            long= "type",
            default_value = "Morpheme",
            help = "Type of syllable break to perform (P/Phoneme or M/Morpheme)",
            long_help = "Specify the type of syllable break to perform. Options are 'P'/'Phoneme' or 'M'/'Morpheme'. Default is 'Morpheme'."
        )]
        break_type: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Syllablebreak {separator, input, output , break_type} => {
            // Check if input file is provided
            let input_path = match input {
                Some(path) => path,
                None => {
                    eprintln!(
                        "Error: No input file specified. Please provide an input file with --input."
                    );
                    std::process::exit(1);
                }
            };

            // Read input from the specified file
            let input_text = match fs::read_to_string(input_path) {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Error reading input file: {}", e);
                    std::process::exit(1);
                }
            };

            // Process text (assuming updated function signature)
            let result = match break_type.to_lowercase().chars().next() {
                Some('p') => syllable_break_phoneme(&input_text, Some(separator)),
                Some('m') => syllable_break(&input_text, Some(separator)),
                _ => {
                    eprintln!("Invalid break type specified. Use 'Phoneme' (or 'P') or 'Morpheme' (or 'M').");
                    std::process::exit(1);
                }
            };

            // Determine output path (use 'result.txt' if not provided)
            let output_path = output
                .clone()
                .unwrap_or_else(|| PathBuf::from("result.txt"));

            // Write output
            match File::create(&output_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(result.as_bytes()) {
                        eprintln!("Error writing to output file: {}", e);
                        std::process::exit(1);
                    }
                    println!("Output written to: {:?}", output_path);
                }
                Err(e) => {
                    eprintln!("Error creating output file: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
