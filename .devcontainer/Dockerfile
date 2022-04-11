FROM debian:bullseye
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
RUN apt-get update \
    && apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev \
    python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts
RUN pip3 install websockets==10.2
ARG CONTAINER_USER
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain nightly-2022-03-10 -y
ENV PATH=${PATH}:$HOME/.cargo/bin:$HOME/.cargo/bin
RUN $HOME/.cargo/bin/rustup component add rust-src --toolchain nightly-2022-03-10
RUN $HOME/.cargo/bin/rustup target add riscv32i-unknown-none-elf
RUN $HOME/.cargo/bin/cargo install cargo-generate espmonitor bindgen ldproxy
RUN $HOME/.cargo/bin/cargo install cargo-espflash --version 1.4.0
ENV ESP_IDF_TOOLS_INSTALL_DIR=global
ENV ESP_BOARD=esp32c3
ENV ESP_IDF_VER=v4.4
ENV ESP_IDF_VERSION=release/v4.4
ENV ESP_IDF_BRANCH=release/v4.4
RUN git clone --recursive --depth 1 --shallow-submodules -b ${ESP_IDF_BRANCH} \
    https://github.com/espressif/esp-idf.git $HOME/esp-idf
RUN $HOME/esp-idf/install.sh ${ESP_BOARD}
ENV IDF_PATH=$HOME/esp-idf
RUN git clone https://github.com/ferrous-systems/espressif-trainings.git && \
    # Hardware Check
    cd $HOME/espressif-trainings/intro/hardware-check && \
    $HOME/.cargo/bin/cargo fetch && \
    $HOME/.cargo/bin/cargo build && \
    $HOME/.cargo/bin/cargo clean && \
    # Http Client
    cd $HOME/espressif-trainings/intro/http-client/exercise && \
    $HOME/.cargo/bin/cargo fetch && \
    cd $HOME/espressif-trainings/intro/http-client/solution && \
    $HOME/.cargo/bin/cargo fetch && \
    # Http Server
    cd $HOME/espressif-trainings/intro/http-server/exercise && \
    $HOME/.cargo/bin/cargo fetch && \
    cd $HOME/espressif-trainings/intro/http-server/solution && \
    $HOME/.cargo/bin/cargo fetch && \
    # MQTT
    cd $HOME/espressif-trainings/intro/mqtt/exercise && \
    $HOME/.cargo/bin/cargo fetch && \
    cd $HOME/espressif-trainings/intro/mqtt/solution && \
    $HOME/.cargo/bin/cargo fetch && \
    cd $HOME/espressif-trainings/intro/mqtt/host-client && \
    $HOME/.cargo/bin/cargo fetch && \
     # Button Interrupt
    cd $HOME/espressif-trainings/advanced/button-interrupt/exercise && \
    $HOME/.cargo/bin/cargo fetch && \
    cd $HOME/espressif-trainings/advanced/button-interrupt/solution && \
    $HOME/.cargo/bin/cargo fetch && \
    # I2C Driver
    cd $HOME/espressif-trainings/advanced/i2c-driver/solution && \
    $HOME/.cargo/bin/cargo fetch && \
    # I2C Sensor Reading
    cd $HOME/espressif-trainings/advanced/i2c-sensor-reading/solution && \
    $HOME/.cargo/bin/cargo fetch && \
    # Delete the repository
    rm -rf $HOME/espressif-trainings