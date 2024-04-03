#/bin/bash

if [ "$(id -u)" != "0" ]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

INIT=$(ps --no-headers -o comm 1)

chmod +x services/systemd/batt_log.service
chmod +x services/runit/run

cargo build --release

if [ $? -ne 0 ]; then
    echo "Cargo build failed, exiting."
    exit 1
fi

cp target/release/batt_log /usr/local/bin/

echo "Installed batt_log to /usr/local/bin/"
echo "You can now run batt_log from the terminal."

echo "Detected init: $INIT"
read -p "Do you want to install batt_log as a service? [Y/n]: " install_service

if [ "$install_service" = "" ]; then
    install_service="y"
fi

if [ "$install_service" != "y" ]; then
    exit 0
fi

if [ "$INIT" = "systemd" ]; then 
    cp services/systemd/batt_log.service /etc/systemd/system/

    systemctl daemon-reload
    systemctl enable batt_log
    systemctl restart batt_log
elif [ "$INIT" = "runit"  ];
then
    cp -r services/runit /etc/sv/batt_log
    ln -sv /etc/sv/batt_log /var/service
    sv up batt_log
fi


echo "Installed batt_log as a service successfully."
