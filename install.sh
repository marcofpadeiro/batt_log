#/bin/bash

if [ "$(id -u)" != "0" ]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

if [ ! -x "$(command -v cargo)" ]; then
    echo "Cargo is not installed. Please install Rust and Cargo before running this script."
    exit 1
fi

# Will either be systemd, runit, or init
INIT_SYSTEM=$(ps --no-headers -o comm 1)

if ! cargo build --release; then
    echo "Cargo build failed, exiting."
    exit 1
fi

cp target/release/batt_log /usr/local/bin/

mkdir -p /etc/batt_log
cp etc/config.toml /etc/batt_log/

mkdir -p /var/lib/batt_log
chown $(whoami):$(whoami) /var/lib/batt_log
chmod u+rw /var/lib/batt_log
chmod g+rw /var/lib/batt_log

echo "Installed batt_log to /usr/local/bin/batt_log"
echo "You can now run batt_log from the terminal."

echo "Detected init: $INIT_SYSTEM"
read -p "Do you want to install batt_log as a service? [Y/n]:" install_service
    
if [[ ! "${install_service,,}" =~ ^(y|)$ ]]; then
    exit 0
fi

declare -A services
services=(["systemd"]="systemd" ["runit"]="runit" ["init"]="openrc")

systemd() {
    chmod +x contrib/systemd/batt_log.service
    cp contrib/systemd/batt_log.service /etc/systemd/system/
    systemctl daemon-reload
    systemctl enable batt_log
}

runit() {
    chmod +x contrib/runit/run
    cp -r contrib/runit /etc/sv/batt_log
    ln -sv /etc/sv/batt_log /var/service
}

openrc() {
    chmod +x contrib/openrc/batt_log
    cp contrib/openrc/batt_log /etc/init.d/
    rc-update add batt_log default
}

${services[$INIT_SYSTEM]}

echo "Installed batt_log as a service successfully."
