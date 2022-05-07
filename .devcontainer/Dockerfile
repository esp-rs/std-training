FROM debian:bullseye
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ARG CONTAINER_USER=esp
ARG CONTAINER_GROUP=esp
ARG NIGHTLY_VERSION=nightly-2022-03-10
RUN apt-get update \
    && apt-get install -y vim nano git curl gcc libudev-dev \
    python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}
ENV PATH=${PATH}:$HOME/.cargo/bin
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain ${NIGHTLY_VERSION} -y \
    && $HOME/.cargo/bin/rustup component add rust-src --toolchain ${NIGHTLY_VERSION} \
    && $HOME/.cargo/bin/rustup target add riscv32i-unknown-none-elf \
    && $HOME/.cargo/bin/cargo install cargo-generate cargo-espflash espmonitor bindgen ldproxy
ENV ESP_BOARD=esp32c3
ENV ESP_IDF_VERSION=release/v4.4
ENV ESP_IDF_TOOLS_INSTALL_DIR=global
RUN mkdir -p .espressif/frameworks/ \
    && git clone --branch ${ESP_IDF_VERSION} -q --depth 1 --shallow-submodules \
    --recursive https://github.com/espressif/esp-idf.git \
    .espressif/frameworks/esp-idf-v4.4 \
    && python3 .espressif/frameworks/esp-idf-v4.4/tools/idf_tools.py install cmake ninja \
    && .espressif/frameworks/esp-idf-v4.4/install.sh ${ESP_BOARD} \
    && rm -rf .espressif/dist \
    && rm -rf .espressif/frameworks/esp-idf-v4.4/docs
ENV IDF_TOOLS_PATH=/home/${CONTAINER_USER}/.espressif
RUN echo "source /home/${CONTAINER_USER}/.espressif/frameworks/esp-idf-v4.4/export.sh" >> ~/.bashrc
CMD "/bin/bash"