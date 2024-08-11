pub mod configuration;
pub mod converter;
pub mod karabiner;
pub mod keys;

use std::io::Read;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use karabiner::KarabinerConfig;

use crate::configuration::Configuration;
use crate::converter::convert_configuration;

const KARABINER_CONFIG: &str = "~/.config/karabiner/karabiner.json";

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
                Method::Extend => extend_config(karabiner_config),
                Method::Replace => replace_karabiner_config(karabiner_config),
                Method::StdOut => {
                    let json = serde_json::to_string_pretty(&karabiner_config)?;
                    println!("{}", json);
                    Ok(())
                }
            }
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

fn replace_karabiner_config(karabiner_config: KarabinerConfig) -> anyhow::Result<()> {
    let mut config = get_karabiner_config_from_file()?;

    let complex_rules = karabiner_config.profiles[0]
        .complex_modifications
        .rules
        .clone();

    Ok(())
}

fn extend_config(karabiner_config: KarabinerConfig) -> anyhow::Result<()> {
    todo!()
}

fn get_karabiner_config<R: Read>(mut reader: R) -> Result<KarabinerConfig> {
    let mut config = String::new();
    reader
        .read_to_string(&mut config)
        .context("Failed to read configuration data")?;
    let value = serde_json::from_str(&config)?;

    let config: KarabinerConfig = serde_json::from_value(value)?;
    Ok(config)
}
fn get_karabiner_config_from_file() -> Result<KarabinerConfig> {
    let file = std::fs::File::open(KARABINER_CONFIG).context(format!(
        "Could not find karabiner configuration. Expected it to be in: {}",
        &KARABINER_CONFIG
    ))?;
    get_karabiner_config(file)
}

fn replace_config(
    karabiner_config: KarabinerConfig,
    config: Configuration,
) -> Result<KarabinerConfig> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn mock_config_str() -> &'static str {
        r#"{
            "profiles": [
                {
                    "complex_modifications":{} ,
                    "name": "Default profile",
                    "selected": true
                }
            ]
        }"#
    }

    #[test]
    fn test_get_karabiner_config_valid() -> anyhow::Result<()> {
        let reader = Cursor::new(mock_config_str());

        let result = get_karabiner_config(reader)?;
        // assert_eq!(result["profiles"][0]["name"], "Default profile");
        Ok(())
    }
}
