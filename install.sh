#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo_failure() {
    echo -e >&2 "${RED}:: $1${NC}"
    exit 1
}

copy_file() {
    cp "$1" "$2" 2>&1 || echo_failure "Failed to copy $1 to $2."
    echo -e "${GREEN}:: Successfully${NC} copied $1 to $2."
}

initialize_database() {
    mkdir -p /var/log/batt_log ||  echo_failure "Failed to create /var/log/batt_log."
    chown :$(logname) /var/log/batt_log &&
    chmod u+rw,g+rw /var/log/batt_log &&
    echo -e "${CYAN}:: Initialized${NC} /var/log/batt_log/"
}

install_service() {
    case $1 in
        "systemd")
            chmod +x contrib/systemd/batt_log.service &&
            cp contrib/systemd/batt_log.service /etc/systemd/system/ &&
            systemctl daemon-reload &&
            systemctl enable batt_log
            ;;
        "runit")
            chmod +x contrib/runit/run && 
            cp -r contrib/runit /etc/sv/batt_log && 
            ln -sv /etc/sv/batt_log /var/service
            ;;
        "init")
            cp contrib/openrc/batt_log /etc/init.d/ && 
            echo -e "command_user=\"$(logname):$(whoami)\"" >> /etc/init.d/batt_log && 
            chmod +x /etc/init.d/batt_log && 
            rc-update add batt_log default
            ;;
        *)
            echo_failure "Unsupported init system: $service_method"
            ;;
    esac
    echo -e ":: Installed batt_log as a service ${GREEN}successfully${NC}."
}

install_tui() {
    [ ! -f target/release/batt_log-tui ] && echo_failure "batt_log-tui binary not found. Please run ${CYAN}'cargo build --release --bin batt_log-tui'${RED} first."
    copy_file "target/release/batt_log-tui" "/usr/local/bin/batt_log-tui"
}

ask() {
    local answer
    echo
    read -p "$1 [Y/n]:" answer

    if [[ "${answer,,}" =~ ^(y|)$ ]]; then
        shift
        $@
    fi
}

main() {
    if [ "$(id -u)" != "0" ]; then
        echo_failure "This script must be run as root"
    fi

    local init_system=$(ps --no-headers -o comm 1)
    echo -e ":: Detected init: ${CYAN}$init_system${NC}\n"

    [ ! -f target/release/batt_log-daemon ] && echo_failure "batt_log-daemon binary not found. Please run ${CYAN}'cargo build --release'${RED} first."

    copy_file "target/release/batt_log-daemon" "/usr/local/bin/batt_log-daemon"

    mkdir -p /etc/batt_log
    copy_file "etc/config.toml" "/etc/batt_log/config.toml"

    initialize_database

    ask "Do you want to install batt_log-tui to read the logs?" install_tui
    ask "Do you want to install batt_log as a service?" install_service "$init_system"
}

main "$@"
