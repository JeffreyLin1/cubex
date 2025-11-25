# Cubex - 3D solvalble rubiks cube in the terminal on rust

![Screen Recording 2025-11-20 at 8 33 45 PM](https://github.com/user-attachments/assets/55590869-c1b3-4abb-a94b-9936aa22415c)

inspired by [this guys C cube](https://github.com/been-jamming/rubiks_cube/tree/master) and [this guys rust cube]()
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
