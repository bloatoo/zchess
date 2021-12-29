# zchess

A terminal interface for chess written in Rust. You can use it to play chess either locally, or against other players via the Lichess API.

### Installation
Build from source:
```
git clone https://github.com/bloatoo/chess
cd chess
cargo install --path .
```

Dependencies: `cargo` and `rustc`. <br />
The binary gets installed to `~/.cargo/bin/zch`, make sure `~/.cargo/bin` is in PATH.

### Configuration
An example configuration file has been provided in the GitHub repository. Move/copy that file to `~/.config/zchess.toml` and configure it to your liking.

### Default Keybinds

```
q | Quit
hjkl + arrow keys | Move the cursor during games
enter | Select a menu item or a piece, or move the selected piece
```
### Preview

![Preview](media/preview.png?raw=true "Preview")
