#/bin/bash

if [ "$(id -u)" != "0" ]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

cargo build --release

if [ $? -ne 0 ]; then
    echo "Cargo build failed, exiting."
    exit 1
fi

cp target/release/batt_log /usr/local/bin/

echo "Installed batt_log to /usr/local/bin/"
echo "You can now run batt_log from the terminal."

read -p "Do you want to install batt_log as a service? [y/n]: " install_service

if [ "$install_service" != "y" ]; then
    exit 0
fi

echo "[Unit]
Description=batt_log service

[Service]
ExecStart=/usr/local/bin/batt_log
Restart=always
User=$(whoami)

[Install]
WantedBy=multi-user.target
" > batt_log.service

chmod +x batt_log.service
mv batt_log.service /etc/systemd/system/

systemctl enable batt_log

echo "Installed batt_log as a service."
echo "On next boot, batt_log will start automatically."

