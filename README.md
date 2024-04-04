# batt_log
A Linux laptop battery logging tool written in Rust.

It was built to track power usage of a laptop battery during each session. It keeps tracks of capacity (%), power usage (Watts).

The main goal of this project is to offer a tool that works on any distibution with systemd, runit or openrc.

## Installation
The installation script will take care of copying the binary to `/usr/local/bin` and create a service.
It will also create a file `/var/log/batt_log.db` to store the logs.

To install the tool, you can run the following commands:
```bash
cargo build --release
./install.sh
```

## Configuration
The default configuration file is located at `/etc/batt_log/config.toml`, it follows the TOML file format. 
You can copy this file to your home directory to override the default configuration, the order it will look for a configuration is the following:
- `~/.config/batt_log/config.toml`
- `~/.config/batt_log.toml`
- `~/.batt_log.toml`
- `/etc/batt_log/config.toml`


### Configuration options

- `polling_interval`: The interval in seconds between each log. Default is 60 seconds.
- `log_file`: The path to the log file. Default is `/var/log/batt.log`.


## Uninstall
To uninstall the tool, you can run the following command:
```bash
./uninstall.sh
```
