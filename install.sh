#!/bin/bash

echo_failure() {
    echo >&2 ":: $1"
    exit 1
}

copy_file() {
    cp "$1" "$2" 2>&1 || echo_failure ":: Failed to copy $1 to $2."
    echo ":: Successfully copied $1 to $2."
}

initialize_database() {
    touch /var/log/batt_log.db || echo_failure ":: Failed to create /var/log/batt_log.db."
    chown :$(logname) /var/log/batt_log.db &&
    chmod u+rw,g+rw /var/log/batt_log.db &&
    echo ":: Initialized /var/log/batt_log.db."
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
            echo "command_user=\"$(logname):$(whoami)\"" >> /etc/init.d/batt_log && 
            chmod +x /etc/init.d/batt_log && 
            rc-update add batt_log default
            ;;
        *)
            echo_failure "Unsupported init system: $service_method"
            ;;
    esac
    echo ":: Installed batt_log as a service successfully."
}

main() {
    if [ "$(id -u)" != "0" ]; then
        echo_failure "This script must be run as root"
    fi

    local init_system=$(ps --no-headers -o comm 1)
    echo -e ":: Detected init: $init_system\n"

    [ ! -f target/release/batt_log ] && echo_failure "batt_log binary not found. Please run 'cargo build --release' first."

    copy_file "target/release/batt_log" "/usr/local/bin/"

    mkdir -p /etc/batt_log
    copy_file "etc/config.toml" "/etc/batt_log/config.toml"

    initialize_database

    echo ""
    read -p "Do you want to install batt_log as a service? [Y/n]:" install_service_answer
    if [[ "${install_service_answer,,}" =~ ^(y|)$ ]]; then
        install_service "$init_system"
    else
        exit 0
    fi
}

main "$@"
