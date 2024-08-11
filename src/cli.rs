pub mod configuration;
pub mod converter;
pub mod karabiner;
pub mod keys;

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};

use crate::configuration::Configuration;
use crate::converter::convert_configuration;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug, Clone, ValueEnum)]
pub enum Method {
    Extend,
    Replace,
    StdOut,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Creates a new karabiner configuration file.
    Create {
        /// Toml file containing the configuration.
        file: String,

        #[arg(short, long, default_value = "std-out")]
        method: Method,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Create { file, method }) => {
            let config = read_config(&file)?;
            let karabiner_config = convert_configuration(&config);

            match method {
                Method::Extend => {
                    todo!()
                }
                Method::Replace => {
                    todo!()
                }
                Method::StdOut => {
                    let json = serde_json::to_string_pretty(&karabiner_config)?;
                    println!("{}", json);
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

fn read_config(file: &str) -> anyhow::Result<Configuration> {
    let config = std::fs::read_to_string(file)?;
    let config: toml::Value = toml::from_str(&config)?;
    let config = Configuration::from_toml(&config).context("Invalid configuration file.")?;
    Ok(config)
}
