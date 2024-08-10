pub mod configuration;
pub mod converter;
pub mod karabiner;
pub mod keys;

pub use configuration::Configuration;
use toml::Value;

fn main() -> anyhow::Result<()> {
    let toml_str = r#"
    [remaps]
    caps_lock = "hyper"

    [layers]
    layer1 = "hyper"
    layer2 = "hyper+v"

    # By default commands return to the base layer. Remaps remain in the current layer.
    [layer1]
    h = { command = "hello" }
    a = { remap =  "ctrl+shift+left_arrow"}
    y = { remap = "ctrl+shift+up_arrow" }
    n = { remap = "ctrl+shift+down_arrow", target_layer = "layer2", description = "These arguments are optional" }
    esc = { move_layer = "base"}

    [layer2]
    a = { command = "app_launcher" }
    "#;

    let toml_value: Value = toml_str.parse()?;
    let config = Configuration::from_toml(&toml_value)?;
    println!("{:?}", config);

    let converted_config = converter::convert_configuration(&config);

    let json = serde_json::to_string_pretty(&converted_config)?;
    println!("{}", json);

    Ok(())
}
