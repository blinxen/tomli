mod delete;
mod errors;
mod parser;
mod query;
mod set;

use std::str;
use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use errors::TomliError;
use toml_edit::DocumentMut;

#[derive(Parser)]
#[command(version)]
/// Simple CLI for quering and editing TOML files
///
/// By default, the input is read from STDIN unless --filepath is specified.
struct Cli {
    /// Path to a TOML file
    #[arg(short, long, global = true)]
    filepath: Option<PathBuf>,
    /// Edit file in-place when applicable
    #[arg(short = 'i', long, global = true)]
    in_place: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query a TOML document
    Query {
        /// Query expression
        query: String,
    },
    /// Edit a TOML document
    Set {
        /// Query expression, that specifies which element you want to set / append
        query: String,
        /// Value to set
        value: String,
        /// Value type that should be used
        #[arg(value_enum, short = 't', long = "type", default_value_t = ValueType::Str)]
        value_type: ValueType,
    },
    /// Delete an item in a TOML document
    Delete {
        /// If this flag is set, the process will exit with 0 even if the key does not exist
        #[arg(short = 'e', long)]
        if_exists: bool,
        /// Query expression, that specifies which element you want to set / append
        query: String,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ValueType {
    Str,
    Int,
    Float,
    Bool,
}

// Read the input document from a file
// If the user did not provide a file, then read from stdin
fn read_input(filepath: &Option<PathBuf>) -> Result<DocumentMut, TomliError> {
    let input = if let Some(filepath) = filepath {
        std::fs::read_to_string(filepath)?
    } else {
        std::io::read_to_string(std::io::stdin())?
    };

    Ok(input.parse::<DocumentMut>()?)
}

fn main() {
    let cli = Cli::parse();
    let mut document = read_input(&cli.filepath).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let (query, result, can_write) = match cli.command {
        Commands::Query { query } => (query.clone(), query::exec(&document, &query), false),
        Commands::Set {
            query,
            value,
            value_type,
        } => (
            query.clone(),
            set::exec(&mut document, &query, &value, value_type),
            true,
        ),
        Commands::Delete { if_exists, query } => {
            let mut result = delete::exec(&mut document, &query);
            if if_exists {
                result = Ok(document.to_string())
            }
            (query.clone(), result, true)
        }
    };

    if let Err(err) = result {
        match err {
            TomliError::QuerySyntaxError(position) => {
                eprintln!(
                    "{}:\n\n{}\n{}--^-",
                    err,
                    query,
                    " ".repeat(position.saturating_sub(2)),
                );
            }
            _ => eprintln!("{}", err),
        }
        std::process::exit(1);
    } else if can_write && cli.in_place && cli.filepath.is_some() {
        fs::write(cli.filepath.unwrap(), result.unwrap().as_bytes())
            .expect("An error occured when trying to save the file");
    } else {
        println!("{}", result.unwrap());
    }
}
