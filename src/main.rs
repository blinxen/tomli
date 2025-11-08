mod copy;
mod delete;
mod errors;
mod parser;
mod query;
mod set;

use std::fs::File;
use std::io::Write;
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
        /// Query expression that specifies which element you want to set / append
        query: String,
        /// Value to set
        value: String,
        /// Value type that should be used
        #[arg(value_enum, short = 't', long = "type", default_value_t = ValueType::Str)]
        value_type: ValueType,
        /// Use dotted keys instead of creating a subtable
        ///
        /// By default, tomli expands dotted keys into a subtable.
        ///
        /// For example:
        ///
        /// [dependencies.windows]
        /// workspace = true
        ///
        /// With this flag, tomli will use dotted key notation instead:
        ///
        /// [dependencies]
        /// windows.workspace = true
        #[arg(verbatim_doc_comment, long, default_value_t = false)]
        dotted_key: bool,
    },
    /// Delete an item in a TOML document
    Delete {
        /// If this flag is set, the process will exit with 0 even if the key does not exist
        #[arg(short = 'e', long)]
        if_exists: bool,
        /// Query expression that specifies which element you want to set / append
        query: String,
    },
    /// Copy a item from one TOML document to another
    Copy {
        /// Query expression that specifies which item you want to copy
        source_query: String,
        /// Destination file
        destination: PathBuf,
        /// Query expression that specifies where you want to store the copied struct
        destination_query: String,
        /// Use dotted keys instead of creating a subtable
        ///
        /// By default, tomli expands dotted keys into a subtable.
        ///
        /// For example:
        ///
        /// [dependencies.windows]
        /// workspace = true
        ///
        /// With this flag, tomli will use dotted key notation instead:
        ///
        /// [dependencies]
        /// windows.workspace = true
        #[arg(verbatim_doc_comment, long, default_value_t = false)]
        dotted_key: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ValueType {
    Str,
    Int,
    Float,
    Bool,
    Datetime,
}

// Read the input document from a file
// If the user did not provide a file, then read from stdin
fn read_input(filepath: Option<&PathBuf>) -> Result<DocumentMut, TomliError> {
    let input = if let Some(filepath) = filepath {
        std::fs::read_to_string(filepath)?
    } else {
        std::io::read_to_string(std::io::stdin())?
    };

    Ok(input.parse::<DocumentMut>()?)
}

fn main() {
    let cli = Cli::parse();
    let mut document = read_input(cli.filepath.as_ref()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let (query, result, can_write, filepath) = match cli.command {
        Commands::Copy {
            source_query,
            destination,
            destination_query,
            dotted_key,
        } => {
            let mut destination_document = read_input(Some(&destination)).unwrap_or_default();

            (
                source_query.clone(),
                copy::exec(
                    &document,
                    &source_query,
                    &mut destination_document,
                    &destination_query,
                    dotted_key,
                ),
                true,
                Some(destination.clone()),
            )
        }
        Commands::Query { query } => (
            query.clone(),
            query::exec(&document, &query),
            false,
            cli.filepath,
        ),
        Commands::Set {
            query,
            value,
            value_type,
            dotted_key,
        } => (
            query.clone(),
            set::exec(&mut document, &query, &value, value_type, dotted_key),
            true,
            cli.filepath,
        ),
        Commands::Delete { if_exists, query } => {
            let mut result = delete::exec(&mut document, &query);
            if if_exists {
                result = Ok(document.to_string())
            }
            (query.clone(), result, true, cli.filepath)
        }
    };

    match result {
        Err(error) => {
            match error {
                TomliError::QuerySyntaxError(position) => eprintln!(
                    "{}:\n\n{}\n{}--^-",
                    error,
                    query,
                    " ".repeat(position.saturating_sub(2)),
                ),
                _ => eprintln!("{}", error),
            };
            std::process::exit(1);
        }
        Ok(result) => {
            if can_write
                && cli.in_place
                && let Some(filepath) = filepath
            {
                let mut file =
                    File::create(filepath).expect("An error occured when trying to save the file");
                file.write_all(result.as_bytes())
                    .expect("An error occured when trying to save the file");
            } else if cli.strip_trailing_newline {
                print!("{result}");
            } else {
                println!("{result}");
            }
        }
    };
}
