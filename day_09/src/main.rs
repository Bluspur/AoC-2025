use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
enum Part {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
    Both,
}

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
    #[arg(short, long)]
    input: std::path::PathBuf,
    #[arg(short, long, default_value = "both")]
    part: Part,
}

fn main() -> Result<()> {
    // Get the arguments using Clap.
    let args = Args::parse();
    // Initialize the logger with the verbosity level from the arguments.
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();
    // Read the input file into a string.
    let input = read_input_file_to_string(&args.input)
        .with_context(|| format!("Failed to read input file {}", args.input.display()))?;

    // Run the appropriate part(s) of the solution.
    match args.part {
        Part::One => {
            let part_1 = solution::run_part_1(&input)?;
            println!("Part 1: {}", part_1);
        }
        Part::Two => {
            let part_2 = solution::run_part_2(&input)?;
            println!("Part 2: {}", part_2);
        }
        Part::Both => {
            let part_1 = solution::run_part_1(&input)?;
            let part_2 = solution::run_part_2(&input)?;
            println!("Part 1: {}", part_1);
            println!("Part 2: {}", part_2);
        }
    }

    Ok(())
}

fn read_input_file_to_string(path: &std::path::Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
