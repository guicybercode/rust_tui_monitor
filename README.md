# Rust TUI Monitor

A lightweight and aesthetic terminal-based system monitor built with Rust.

<img src="rust_tui_img.jpg" width="300"/>


## Features

- Real-time system monitoring (CPU, Memory, Disk, Network)
- Extended system information (user, hostname, OS, distribution, kernel, architecture, shell, hardware)
- Multiple aesthetic themes (default, nord, gruvbox, dracula, monokai)
- Customizable via config file
- Clean and minimal design with cute ASCII dog art
- Cross-platform support (Windows, Linux, macOS)
- Low resource usage

## Installation

```bash
cargo build --release
```

On Linux/macOS, you can also use:
```bash
chmod +x build.sh
./build.sh
```

## Usage

Run the monitor:
```bash
cargo run --release
```

Or use the convenience scripts:
- **Windows**: Double-click `run.bat`
- **Linux/macOS**: `chmod +x run.sh && ./run.sh`

With custom theme:
```bash
cargo run --release -- --theme nord
```

With custom config:
```bash
cargo run --release -- --config path/to/config.toml
```

With custom refresh rate:
```bash
cargo run --release -- --refresh-rate 500
```

Run the compiled executable directly:
```bash
.\target\release\rust-tui-monitor.exe
```

## Keybindings

- `Q` - Quit application
- `R` - Force refresh
- `T` - Cycle through themes
- `ESC` - Quit application

## Configuration

Create a `config.toml` file in your config directory or use the one in the project root.

Example configuration:
```toml
refresh_rate = 1000
show_cpu = true
show_memory = true
show_disk = true
show_network = true
rounded_borders = true
temp_unit = "celsius"

[theme]
background = [26, 27, 38]
foreground = [198, 208, 245]
primary = [137, 180, 250]
secondary = [203, 166, 247]
success = [166, 227, 161]
warning = [249, 226, 175]
danger = [243, 139, 168]
border = [88, 91, 112]
```

## Themes

Available themes (13 total):
- `default` - Catppuccin-inspired
- `nord` - Nord color scheme
- `gruvbox` - Gruvbox color scheme
- `dracula` - Dracula color scheme
- `monokai` - Monokai color scheme
- `cyberpunk` - Neon cyberpunk vibes
- `tokyo-night` - Tokyo Night theme
- `solarized-dark` - Solarized Dark
- `solarized-light` - Solarized Light
- `one-dark` - One Dark theme
- `material` - Material Design
- `ayu-dark` - Ayu Dark
- `rosepine` - Rosé Pine theme

## System Information Displayed

### Static Information
- **User and hostname** - Complete user@hostname format
- **Operating system** - OS name and distribution details
- **Kernel version** - Full kernel information
- **Architecture** - x86_64, aarch64, etc.
- **Shell environment** - bash, zsh, PowerShell, etc.
- **CPU model** - Full processor name
- **CPU cores** - Logical and physical core count
- **CPU frequency** - Current processor frequency in GHz
- **Total memory** - RAM capacity in GB
- **System uptime** - Days, hours, minutes
- **Boot time** - Exact system boot timestamp
- **Process count** - Number of running processes
- **Display info** - Resolution and refresh rate (Windows/Linux/macOS)
- **Audio device** - Current audio output device (Windows/Linux/macOS)

### Real-time Monitoring
- **CPU usage** - Global usage percentage with sparkline history graph
- **Memory usage** - Detailed RAM and Swap statistics with usage bars
  - Shows: used GB, free GB, and percentage
- **Disk usage** - Per-disk monitoring with labels
  - HD1, HD2, HD3 automatic labeling
  - Disk type identification (HDD/SSD)
  - File system detection (NTFS, ext4, APFS, etc.)
- **Network statistics** - Real-time network monitoring
  - Download/Upload rates (B/s, KB/s, MB/s)
  - Total data transferred in session

## Requirements

- Rust 1.70 or higher
- Works on Windows, Linux, and macOS

---

여호와는 나의 목자시니 내게 부족함이 없으리로다 - 시편 23:1


