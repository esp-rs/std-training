FROM debian:bullseye
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
RUN apt-get update \
    && apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev \
    python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts
ARG CONTAINER_USER=esp
ENV USER=${CONTAINER_USER}
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}
ARG NIGHTLY_VERSION=nightly-2022-03-10
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain ${NIGHTLY_VERSION} -y
ENV PATH=${PATH}:$HOME/.cargo/bin:$HOME/.cargo/bin
RUN $HOME/.cargo/bin/rustup component add rust-src --toolchain ${NIGHTLY_VERSION}
RUN $HOME/.cargo/bin/rustup target add riscv32i-unknown-none-elf
RUN $HOME/.cargo/bin/cargo install cargo-generate espmonitor bindgen ldproxy
RUN $HOME/.cargo/bin/cargo install cargo-espflash --version 1.4.1
ENV ESP_IDF_TOOLS_INSTALL_DIR=global
ENV ESP_BOARD=esp32c3
ENV ESP_IDF_VER=v4.4
ENV ESP_IDF_VERSION=release/v4.4
ENV ESP_IDF_BRANCH=release/v4.4
RUN git clone --recursive --depth 1 --shallow-submodules -b ${ESP_IDF_BRANCH} \
    https://github.com/espressif/esp-idf.git $HOME/esp-idf
RUN $HOME/esp-idf/install.sh ${ESP_BOARD}
ENV IDF_PATH=$HOME/esp-idf