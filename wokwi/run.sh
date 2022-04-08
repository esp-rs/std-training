#!/bin/bash
set -e
if [ "$USER" = "vscode" ]; then
path="/home/vscode/workspace"
else
path="/workspace/espressif-trainings"
fi
echo Building and running: $CURRENT_PROJECT 
cd $CURRENT_PROJECT
cargo espflash --release save-image app.bin
pip3 install -r $path/wokwi/requirements.txt
python3 $path/wokwi/wokwi-server-esp32c3.py