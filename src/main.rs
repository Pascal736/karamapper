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
    layer1 = "l+left_command+left_shift+left_option+left_control"
    layer2 = "v+left_command+left_shift+left_option+left_control"

    [layer1]
    escape = { move_layer= "baselayer" }
    o = { command = "Open -a '1Password.app'", next_layer= "baselayer" }
    b = { command = "Open -a 'Brave Browser.app'", next_layer= "baselayer" }
    z = { command = "Open -a 'Zed.app'", next_layer= "baselayer" }
    g = { command = "Open -a 'ChatGPT.app'", next_layer= "baselayer" }
    k = { command = "Open -a 'kitty.app'", next_layer= "baselayer" }
    s = { command = "Open -a 'Slack.app'", next_layer= "baselayer" }
    n = { command = "Open -a 'Notion.app'", next_layer= "baselayer" }
    w = { command = "Open -a 'Warp.app'", next_layer= "baselayer" }
    m = { command = "Open -a 'WhatsApp.app'", next_layer= "baselayer" }

    [layer2]
    escape = { move_layer= "baselayer" }
    m = { remap= "m+left_command", next_layer= "baselayer", description="Minimize Window" }
    "#;

    let toml_value: Value = toml_str.parse()?;
    let config = Configuration::from_toml(&toml_value)?;

    let converted_config = converter::convert_configuration(&config);

    let json = serde_json::to_string_pretty(&converted_config)?;
    println!("{}", json);

    Ok(())
}
