pub mod configuration;
pub mod karabiner;
pub mod keys;
pub mod parser;

pub use configuration::Configuration;
use toml::Value;

fn main() {
    let toml_str = r#"
    [remaps]
    caps_lock = "hyper"

    [commands]
    hello = "sh -c \"echo Hello World\""
    hello2 = "sh -c \"echo Hello World 2\""
    app_launcher = "apple:launchpad"

    [layers]
    layer1 = "hyper"
    layer2 = "hyper+v"

    [layer1]
    h = "hello"
    y = { command = "hello2", target_layer = "base", description = "These arguments are optional" }

    [layer2]
    a = "app_launcher"
    "#;

    let toml_value: Value = toml_str.parse().unwrap();
    let config = Configuration::from_toml(&toml_value).unwrap();

    println!("{:?}", config);
}
