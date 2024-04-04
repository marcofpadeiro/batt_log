#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo_failure() {
    echo >&2 ":: $1"
    exit 1
}

remove_file() {
    rm -rf "$1" 2>&1 || echo_failure "${RED}:: Failed${NC} to remove $1."
    echo -e "${GREEN}:: Successfully${NC} removed $1."
}

uninstall_service() {
    case $1 in
        "systemd")
            systemctl stop batt_log
            systemctl disable batt_log
            remove_file "/etc/systemd/system/batt_log.service"
            systemctl daemon-reload
            ;;
        "runit")
            sv down batt_log
            rm -rf /etc/sv/batt_log
            remove_file "/var/service/batt_log"
            ;;
        "init")
            rc-service batt_log stop
            rc-update del batt_log default
            remove_file "/etc/init.d/batt_log"
            ;;
        *)
            echo_failure "Unsupported init system: $init_system"
            ;;
    esac
    echo -e ":: Uninstalled batt_log service ${GREEN}successfully${NC}."
}

confirm_uninstall() {
    echo -e ":: Detected init: ${CYAN}$init_system${NC}\n"
    read -p "Are you sure you want to uninstall batt_log? [Y/n]:" confirm
    
    if [[ ! "${confirm,,}" =~ ^(y|)$ ]]; then
        exit 0
    fi
}

delete_logs_and_configs() {
    echo -e ""
    read -p "Do you wish to delete the database logs and config files? [Y/n]:" confirm

    if [[ "${confirm,,}" =~ ^(y|)$ ]]; then
        remove_file "/var/log/batt_log" &&
        remove_file "/etc/batt_log" && 
        remove_file "/home/$(logname)/.config/batt_log" 
    fi
}

main() {
    if [ "$(id -u)" != "0" ]; then
        echo_failure "This script must be run as root"
    fi

    local init_system=$(ps --no-headers -o comm 1)
    
    confirm_uninstall

    remove_file "/usr/local/bin/batt_log-daemon"
    
    uninstall_service "$init_system"

    delete_logs_and_configs

    echo -e "\n:: Removed batt_log ${GREEN}successfully${NC}."
}

main "$@"

