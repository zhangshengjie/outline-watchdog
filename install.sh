#!/bin/bash
###
 # @Description: 
 # @Version: 1.0
 # @Autor: z.cejay@gmail.com
 # @Date: 2022-11-02 23:36:13
 # @LastEditors: cejay
 # @LastEditTime: 2022-11-03 00:30:18
### 

set -euo pipefail

cat <<EOF
Usage: install.sh --zoneid <zoneid> --hostname <hostname> --access_key_id <access_key_id> --secret_access_key <secret_access_key>

  --zoneid              AWS Route53 Zone ID
  --hostname            your hostname in AWS Route53
  --access_key_id       AWS Access Key ID
  --secret_access_key   AWS Secret Access Key
EOF

# get parameters
while [[ $# -gt 0 ]]; do
  key="$1"
  case $key in
    --zoneid)
      ZONEID="$2"
      shift
      shift
      ;;
    --hostname)
      HOSTNAME="$2"
      shift
      shift
      ;;
    --access_key_id)
      ACCESS_KEY_ID="$2"
      shift
      shift
      ;;
    --secret_access_key)
      SECRET_ACCESS_KEY="$2"
      shift
      shift
      ;;
    # --region)
    #   REGION="$2"
    #   shift
    #   shift
    #   ;;
    *)
      echo "Unknown parameter: $key"
      exit 1
      ;;
  esac
done

# print parameters
echo "ZONEID = ${ZONEID}"
echo "HOSTNAME = ${HOSTNAME}"
echo "ACCESS_KEY_ID = ${ACCESS_KEY_ID}"
echo "SECRET_ACCESS_KEY = ${SECRET_ACCESS_KEY}"
#echo "REGION = ${REGION}"

sudo apt update
sudo apt install -y curl wget git

exit 0

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# git clone
cd /root/
git clone https://github.com/zhangshengjie/outline-watchdog.git

# build binary
cd outline-watchdog
cargo build --release

# binary path: ./target/release/outline-watchdog

# create auto start script
cat <<EOF > /etc/systemd/system/outline-watchdog.service
[Unit]
Description=outline-watchdog
Documentation=https://github.com/zhangshengjie/outline-watchdog
After=network.target

[Service]
Type=oneshot
ExecStart=/root/outline-watchdog/target/release/outline-watchdog --zoneid ${ZONEID} --hostname ${HOSTNAME} --access-key-id ${ACCESS_KEY_ID} --secret-access-key ${SECRET_ACCESS_KEY} --region ${REGION}

[Install]
WantedBy=multi-user.target
EOF


# enable auto start
systemctl enable outline-watchdog.service

# start service
systemctl start outline-watchdog.service

# check status
#systemctl status outline-watchdog.service

# install outline vpn
sudo bash -c "$(wget -qO- https://raw.githubusercontent.com/zhangshengjie/outline-watchdog/main/install_server.sh)" --hostname ${HOSTNAME}



