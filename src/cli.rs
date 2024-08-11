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

const KARABINER_CONFIG: &str = ".config/karabiner/karabiner.json";

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
    let existing_config = get_karabiner_config_from_file()?;
    let config_updated = replace_config(existing_config, karabiner_config)?;
    write_karabiner_config(config_updated)?;
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

fn karabiner_config_path() -> anyhow::Result<String> {
    let home_dir = home::home_dir().context("Could not find home directory")?;
    let config_path = home_dir.join(KARABINER_CONFIG).clone();
    let config_str = config_path
        .to_str()
        .context("Could not convert path to string")?;
    Ok(config_str.to_owned())
}

fn get_karabiner_config_from_file() -> Result<KarabinerConfig> {
    let path = karabiner_config_path()?;
    let file = std::fs::File::open(path).context(format!(
        "Could not find karabiner configuration. Expected it to be in: {}",
        &KARABINER_CONFIG
    ))?;
    get_karabiner_config(file)
}
fn replace_config(
    mut existing_config: KarabinerConfig,
    config: KarabinerConfig,
) -> Result<KarabinerConfig> {
    if let (Some(current_profile), Some(new_profile)) = (
        existing_config.profiles.first_mut(),
        config.profiles.first(),
    ) {
        current_profile.complex_modifications = new_profile.complex_modifications.clone();
    }
    Ok(existing_config)
}

fn write_karabiner_config(config: KarabinerConfig) -> Result<()> {
    let json = serde_json::to_string_pretty(&config)?;
    let path = karabiner_config_path()?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use karabiner::{
        ComplexModifications, FromKeyMapping, ManipulationTarget, Manipulator, Profile, Rule,
        ToKeyMapping,
    };
    use keys::Key;

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
        assert_eq!(result.profiles.first().unwrap().name, "Default profile");
        Ok(())
    }

    // #[test]
    // fn test_replace_config() -> anyhow::Result<()> {
    //     let karabiner_config = get_karabiner_config(mock_config_str())?;

    //     let config = KarabinerConfig {
    //         profiles: vec![Profile {
    //             name: "Default profile".to_string(),
    //             selected: true,
    //             devices: None,
    //             complex_modifications: {
    //                 ComplexModifications {
    //                     rules: Some(vec![Rule {
    //                         enabled: true,
    //                         description: Some("Test".to_string()),
    //                         manipulators: vec![Manipulator {
    //                             conditions: None,
    //                             from: FromKeyMapping {
    //                                 key_code: Key::A,
    //                                 modifiers: None,
    //                             },
    //                             to: ManipulationTarget {
    //                                 Key

    //                                 }
    //                             manipulator_type: "basic".to_string(),
    //                             to_if_alone: None,
    //                             to_after_key_up: None,
    //                             to_delayed_action: None,
    //                         }],
    //                     }]),
    //                 }
    //             },
    //         }],
    //     };
    // }
}
