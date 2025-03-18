mod delete;
mod errors;
mod parser;
mod query;
mod set;

use std::path::PathBuf;
use std::str;

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

    let (query, result) = match cli.command {
        Commands::Query { query } => (query.clone(), query::exec(document, &query)),
        Commands::Set {
            query,
            value,
            value_type,
        } => (
            query.clone(),
            set::exec(document, &query, &value, value_type),
        ),
        Commands::Delete { if_exists, query } => {
            let mut result = delete::exec(&mut document, &query);
            if if_exists {
                result = Ok(())
            }
            if result.is_ok() {
                println!("{document}");
            }
            (query.clone(), result)
        }
    };

    result.unwrap_or_else(|err| {
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
    })
}
