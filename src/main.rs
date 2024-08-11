pub mod configuration;
pub mod converter;
pub mod karabiner;
pub mod keys;

pub use configuration::Configuration;
use toml::Value;

fn main() -> anyhow::Result<()> {
    let toml_str = r#"
    [simple_remaps]
    caps_lock = "escape"

    [baselayer]
    caps_lock = { remap = "left_command+left_shift+left_option+left_control"}

    [layers]
    layer1 = "q+left_command+left_shift+left_option+left_control"
    layer2 = "v+left_command+left_shift+left_option+left_control"

    [layer1]
    s = { command = "Open -a '1Password.app'", next_layer= "base" }

    [layer2]
    a = { command = "launchpad" }
    "#;

    let toml_value: Value = toml_str.parse()?;
    let config = Configuration::from_toml(&toml_value)?;

    let converted_config = converter::convert_configuration(&config);

    let json = serde_json::to_string_pretty(&converted_config)?;
    println!("{}", json);

    Ok(())
}
