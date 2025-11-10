# Termsweeper

A simple terminal-based Minesweeper clone I built to learn Rust.

## What This Is
- Terminal Minesweeper with keyboard controls
- Written in Rust using `crossterm` and `tui`
- Includes a basic timer (optionally hidden) end game animations, and modern quality of life features


## Download From Source
```bash
git clone https://github.com/CouchPugtato/Termsweeper
cd Termsweeper
cargo build --release
```


## Run With Inputs From Terminal
The game accepts these CLI options to configure the grid and behavior:

| Option | Alias | Value | Description |
|---|---|---|---|
| `--width` | `-w` | number | Set grid width |
| `--height` | `-h` | number | Set grid height |
| `--difficulty` | `-d` | `easy` / `medium` / `hard` | Set difficulty level |
| `--hide-timer` | `-t` | none | Hide the game clock |

Examples:
```bash
# Run with defaults
cargo run --

# Specify width/height and difficulty
cargo run -- -w 20 -h 12 -d easy

# Hide the timer
cargo run -- -t

# Combine options
cargo run -- -w 30 -h 16 -d hard -t
```

### Difficulty Shortcuts
Use `-d` with one of: `easy`, `medium`, `hard`.

| Difficulty | Shortcut |
|---|---|
| Easy | `-d easy` |
| Medium | `-d medium` |
| Hard | `-d hard` |

## Controls

| Action | Keys |
|---|---|
| Move cursor | Arrow keys, WASD, HJKL |
| Reveal cell | `Enter`, `E` |
| Toggle flag | `Space`, `F` |
| Quit | `Q`, `Esc` |

## Notes
- If your terminal size is large, consider reducing `--width` and `--height`.
- Please add any issues that are found.