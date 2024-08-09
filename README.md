# karamapper


karamapper allows for an easier configuration for the karabiner keymapper.
It has first class support for layers to which commands can be mapped.


## Example Configuration
```toml
[remaps]
caps_lock = "hyper"

[commands]
hello = sh -c "echo Hello World"
hello2 = sh -c "echo Hello World 2"
app_launcher = "apple:launchpad"

[layers]
layer1 = "hyper"
layer2 = "hyper+v"

[layer1]
h= "hello" # if nothing is specified assums command and sets default values for the rest.
y = {command = "hello2", target_layer="base", description="These arguments are optional"}

[layer2]
a = app_launcher

```


## CLI Interface
`paramapper create mapping.toml --replace`
`paramapper create mapping.toml --extend`
