# Pino: Pixel Notification

Pino is a fully customizable notification tool rewritten in Rust. It allows you to display notifications with various options, including dynamic theming, configurable fonts, and system integration.

---

## Shortcuts

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Example: Low Battery Alert](#example-low-battery-alert)
- [Configuration](#configuration)
- [Dependencies](#dependencies)
- [Hardware Usage](#hardware-usage)

---

## Features

- **Customizable Notifications**: Set titles, messages, delay, and fonts.
- **Dynamic Theming with walrs(or pywal)**: Automatically matches the notification theme to your wallpaper.
- **Configurable Settings**: Adjust themes, screen placement, fonts, and more via a TOML config file.
- **Script Integration**: Automate notifications using scripts in any language.

---

## Installation

### Arch Linux

```bash
sudo pacman -S rust
```

### Debian-based Distros

```bash
sudo apt install rustc cargo
```

Then, build the project:

```bash
cargo build --release
```

Copy the binary to `/usr/bin`:

```bash
sudo cp target/release/pino /usr/bin/
```

---

## Dependencies

Pino requires the following dependencies:

- Rust (for building from source)
- Walrs | pywal (optional) for dynamic theming

---

## Usage

Pino supports the following command-line options:

```bash
Usage: pino -t <title> -m <message> [-d <delay>] [-f] [-c <config>]

Options:
  -t, --title       Set the notification title content
  -m, --message     Set the notification message content
  -d, --delay       Set the delay before the program closes (in seconds)
  -f, --font        Print all the available fonts
  -c, --config      Set a custom configuration file
  --help, help      Display usage information
```

### Note:

If you want to insert a new line (wrap text) in the message, use `\n` in the argument parameter.

### Example: Low Battery Alert

You can create a script to notify about low battery status:

```bash
pino -t "Battery Warning" -m "Low battery!\nPlease connect your charger." -d 5
```

---

## Configuration

The app uses a TOML configuration file located at `~/.config/pino/config.toml`. Example:

```toml
[screen]
monitor = 0
horizontal = "left"
vertical = "top"
x = 25
y = 55
width = 300
height = 100
delay = 5

[frame]
fg_color = "#1a1e24"
font_family = "Fira Code"

[border]
weight = 4
color = "#ffffff"
radius = 8

[title]
color = "#c5c6c8"
font_size = 19
x = 4
y = 10

[message]
color = "#626977"
font_size = 15
x = 10
y = 45

[pywal]
pywal = true
background_color  = "bg"
border_color      = "color1"
title_color       = "fg"
message_color     = "color8"

[optional]
sound = false
```

---

## Hardware Usage

Pino is lightweight and efficient. The graphical notification window typically uses approximately **18-25MB of RAM** when active, ensuring minimal system resource consumption.


