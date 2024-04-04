package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/BurntSushi/toml"
)

var CONFIG_PATHS = []string{
	"~/.config/batt_log/config.toml",
	"/etc/batt_log/config.toml",
}

type Config struct {
	DBPath string `toml:"db_path"`
}

func DefaultConfig() *Config {
	return &Config{
		DBPath: "/var/lib/batt_log/log.db",
	}
}

func NewConfig() *Config {
	var config Config
	for _, path := range CONFIG_PATHS {
		expandedPath, err := expandPath(path)
		if err != nil {
			fmt.Fprintln(os.Stderr, "Failed to expand path:", err)
			continue
		}

		if _, err := os.Stat(expandedPath); os.IsNotExist(err) {
			continue
		}

		if _, err := toml.DecodeFile(expandedPath, &config); err != nil {
			fmt.Fprintln(os.Stderr, "Failed to parse config file. Using default configuration.")
			return DefaultConfig()
		}
		return &config
	}

	fmt.Fprintln(os.Stderr, "No valid config file found. Using default configuration.")
	return DefaultConfig()
}

func expandPath(path string) (string, error) {
	if path[:2] == "~/" {
		home, err := os.UserHomeDir()
		if err != nil {
			return "", err
		}
		return filepath.Join(home, path[2:]), nil
	}
	return path, nil
}

