# karamapper

karamapper allows for an easier configuration for the [karabiner-elements](https://github.com/pqrs-org/Karabiner-Elements) keymapper.
It has first class support for layers to which commands can be mapped.
It's aim is not to replace the normal configuration but specicially to make it easier to configure layers.
The whole bandwith of karabiner is not exposed.
It's meant to be use standalone if only layers are needed or in combination with the normal karabiner configuration.

> [!WARNING]
> This is a work in progress. The package is not stable yet.


## Example Configuration
```toml
[baselayer]
caps_lock = { remap = "left_command+left_shift+left_option+left_control"}

[layers]
layer1 = "l+left_command+left_shift+left_option+left_control"
layer2 = "v+left_command+left_shift+left_option+left_control"
layer3 = "m+left_command+left_shift+left_option+left_control"

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
m = { remap = "m+left_command", next_layer= "baselayer", description="Minimize Window" }
t = { remap ="spacebar+left_option+left_shift", description = "Toggle Window Layout" }
h = { remap = "k+left_shift+left_option", description = "Move Foucs left"}
l = { remap = "j+left_shift+left_option", description = "Move Foucs Right"}
y = { remap = "p+left_shift+left_option", description = "Move Foucs Left Screen"}
o = { remap = "n+left_shift+left_option", description = "Move Foucs Right Screen"}

[layer3]
escape = { move_layer= "baselayer" }
key1 = { remap = "key1+left_control+left_shift+left_option+left_control", next_layer= "baselayer", description = "Move to Space 1" }
key2 = { remap = "key2+left_control+left_shift+left_option+left_control", next_layer= "baselayer", description = "Move to Space 2"
```


## CLI Interface
```
paramapper create mapping.toml --method replace # Replaces the configuration in $HOME/.config/karabiner/karabiner.json
paramapper create mapping.toml --method extend # Extends the configuration in $HOME/.config/karabiner/karabiner.json
paramapper create mapping.toml --method stdout # Prints the configuration to stdout
```

## Installation
```bash
cargo install karamapper
```

## TODOs:
-[ ] Implement extend configuration method
-[ ] Add option to target specifc profiles
-[ ] Add backup option
