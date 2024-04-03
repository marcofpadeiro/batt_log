#/bin/bash

if [ "$(id -u)" != "0" ]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

# Will either be systemd, runit, or init
INIT_SYSTEM=$(ps --no-headers -o comm 1)

echo "Detected init: $INIT_SYSTEM"
read -p "Are you sure you want to uninstall batt_log? [Y/n]:" uninstall
    
if [[ ! "${uninstall,,}" =~ ^(y|)$ ]]; then
    exit 0
fi

declare -A services
services=(["systemd"]="systemd" ["runit"]="runit" ["init"]="openrc")

rm /usr/local/bin/batt_log
rm $HOME/.cache/batt.db

systemd() {
    systemctl stop batt_log
    systemctl disable batt_log
    rm /etc/systemd/system/batt_log.service
    systemctl daemon-reload
}

runit() {
    sv down batt_log
    rm -rf /etc/sv/batt_log
    rm /var/service/batt_log
}

openrc() {
    rc-service batt_log stop
    rc-update del batt_log default
    rm /etc/init.d/batt_log
}

${services[$INIT_SYSTEM]}

echo "Removed batt_log successfully."
