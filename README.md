# zchess

`zchess` is a terminal interface for chess written in Rust. You can use it to play chess either locally, or against other players via the Lichess API. <br> <br>
NOTE: This project is an early state of development, so expect bugs. Feel free to open issues and pull requests.

### Features
- Flexible interface configuration
- Online games against real people via Lichess
- Ships a lightweight binary

### Getting started
#### Installation
Building from source:
```
git clone https://github.com/bloatoo/zchess
cd zchess
cargo install --path .
```

You must have the Rust toolchain installed. For an easy way to get Rust installed, visit [this link.](https://rustup.rs/.) <br>
The binary gets installed to `~/.cargo/bin/zch`, so make sure `~/.cargo/bin` is in PATH.

#### Requirements
- A Lichess account and an API key. [Generate an API key here.](https://lichess.org/account/oauth/token)


#### Configuration
An example configuration file has been provided in the GitHub repository. Move/copy that file to `~/.config/zchess.toml` and configure it to your liking. <br>
For Lichess functionality, paste the API key you generated before to the `token` field in the configuration file. The  rest of the configuration should be self-explanatory.

#### Default Keybinds

```
q | Quit
hjkl + arrow keys | Move the cursor during games
a | Abort the current game
r | Resign the current game
enter | Select a menu item or a piece, or move the selected piece
```
### Preview

![Preview](media/preview.png?raw=true "Preview")
