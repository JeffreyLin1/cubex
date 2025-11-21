# Cubex

## Controls

| Action | Keys |
| ------ | ---- |
| Rotate camera horizontally | `Left` / `Right` arrows or `A` / `D` |
| Rotate camera vertically | `Up` / `Down` arrows or `W` / `S` |
| Camera roll | `Q` (counter) / `E` (clockwise) |
| Zoom | `+` / `=` (in), `-` / `_` (out) |
| Face turns | `U`, `R`, `F`, `D`, `L`, `B` (lowercase = clockwise, uppercase = counter-clockwise). Press `'/shift` before a letter for inverse or `2` for double turns. |
| Scramble | `Space` |
| Reset | `X` |
| Quit | `Esc` or `Ctrl+C` |

## Running

```bash
# Run in-place with debug symbols
cargo run

# Run optimized release build
cargo run --release

# Install globally as `cubex`
cargo install --path .
```

After you install just run

```bash
cubex
```