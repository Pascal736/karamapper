pub mod configuration;
pub mod converter;
pub mod karabiner;
pub mod keys;

pub use configuration::Configuration;
use toml::Value;

fn main() {
    let toml_str = r#"
    [remaps]
    caps_lock = "hyper"

    [layers]
    layer1 = "hyper"
    layer2 = "hyper+v"

    # By default commands return to the base layer. Remaps remain in the current layer.
    [layer1]
    h = { command = "hello" }
    y = { command = "hello2", target_layer = "base", description = "These arguments are optional" }
    h = { remap =  "ctrl+shift+left_arrow"}
    l = { remap = "ctrl+shift+right_arrow"}
    y = { remap = "ctrl+shift+up_arrow" }
    n = { remap = "ctrl+shift+down_arrow", target_layer = "layer2", description = "These arguments are optional" }
    q = { move_layer = "layer2" }
    esc = { move_layer = "base"}

    [layer2]
    a = { command = "app_launcher" }
    "#;

    let toml_value: Value = toml_str.parse().unwrap();
    let config = Configuration::from_toml(&toml_value).unwrap();

    println!("{:?}", config);
}
