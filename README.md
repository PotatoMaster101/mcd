# MCD
Small tool to automatically press CTRL and left click in random intervals, used for auto dancing in Minecraft.

## Building
Requires [`cargo`](https://www.rust-lang.org/):
```
$ cargo build --release
```

## Usage
```
$ mcd --help
mcd
Automatically press CTRL and left click in random intervals, use CAPS-LOCK to toggle

USAGE:
    mcd [OPTIONS]

OPTIONS:
    -i, --interval <INTERVAL>    Milliseconds for maximum wait time, defaults to 75
    -k, --key <KEY>              Sneak key to press, defaults to CTRL
        --noclick                Do not use mouse click
    -h, --help                   Print help information
```
