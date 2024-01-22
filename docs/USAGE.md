# Using lymap

When you first open the program, lymap will create a bunch of files in your config directory.

Lymap will look for the XDG_CONFIG_HOME environment variable. If this variable is not set, it will default to the .config directory located inside your user's home directory.

Lymap is configured using JSON files (.json)

## Window Config
This is the config for your window resides. <br>
It is located inside lymap/window_config.json

| Name        | Type       | Description                                                        | Default Value |
|-------------|------------|--------------------------------------------------------------------|---------------|
| bg_colour   | Hex String | This is the background of the window.                              | #0c42a6       |
| font_path   | String     | The absolute path to your desired font. This is an optional value. | Null          |
| font_size   | Int        | The size of the font.                                              | 14            |
| font_colour | Hex String | The colour of the font.                                            | #000000       |
| width       | Int        | The width of the window. Optional                                  | Null          |
| height      | Int        | The height of the window. Optional                                 | Null          |

Though the config shows width and height as null by default, lymap will set itself to 800x600 if either of the values are null inside the config.


## Layouts
a `layouts` directory is created by lymap. This is where you will put your button layouts.
A few examples are located in the [examples](../examples) directory. <br>
A layout is composed of three values.

| Name          | Type             | Description                                          |
|---------------|------------------|------------------------------------------------------|
| idle_colour   | Hex String       | The colour of the buttons while they aren't pressed. |
| active_colour | Hex String       | The colour of the buttons while they are pressed.    |
| buttons       | Array of buttons | The buttons themselves.                              |

Below the buttons are defined.

| Name   | Type   | Description                                     |
|--------|--------|-------------------------------------------------|
| x      | Int    | The x position of the button inside the window. |
| y      | Int    | The y position of the button inside the window. |
| text   | String | The text that will appear inside the button.    |
| key    | String | The key that will activate the button.          |
| width  | Int    | The width of the button.                        |
| height | Int    | The height of the button.                       |

To learn more about the `key` value, take a look inside the [global keybind documentation](./GLOBAL_KEYBINDS.md).

Layouts should be placed in lymap/layouts/your_layout.json, for example.

## Cache
A `cache` directory is crated by lymap. This directory is used by lymap to save old values. <br>
Currently it saves the previously loaded layout and loads it when lymap is next opened. This directory should not be touched by the user.

## Local Keybinds
Lymap offers a few local keybinds, this means it will only listen for this while the main window is focused.

| Keybind  | Description          |
|----------|----------------------|
| Ctrl + L | Load a layout file.  |
| Ctrl + R | Reset loaded layout. |


## Assets
Lymap assets are a bit complicated. Lymap will first look in `/usr/share/lymap` and if nothing is there it will search the binary directory.