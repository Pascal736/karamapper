# karamapper


karamapper allows for an easier configuration for the karabiner keymapper.
It has first class support for layers to which commands can be mapped.

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
`paramapper create mapping.toml --replace`
`paramapper create mapping.toml --extend`
