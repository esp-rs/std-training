#!/bin/bash
set -e
echo Building and running: $CURRENT_PROJECT 
cd $CURRENT_PROJECT
cargo espflash --release save-image app.bin &&
pip3 install -r $HOME/workspace/wokwi/requirements.txt
python3 $HOME/workspace/wokwi/wokwi-server-esp32c3.py