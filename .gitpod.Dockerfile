# Note: gitpod/workspace-base image references older version of CMake, it's necessary to install newer one
FROM  gitpod/workspace-base
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ARG CONTAINER_USER=gitpod
ARG CONTAINER_GROUP=gitpod
ARG NIGHTLY_VERSION=nightly-2022-03-10
ARG ESP_IDF_VERSION=release/v4.4
ARG ESP_BOARD=esp32c3
RUN sudo install-packages git curl gcc clang ninja-build libudev-dev \
    python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 libpython2.7
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain ${NIGHTLY_VERSION} -y \
    && $HOME/.cargo/bin/rustup component add rust-src --toolchain ${NIGHTLY_VERSION} \
    && $HOME/.cargo/bin/rustup target add riscv32imc-unknown-none-elf \
    && $HOME/.cargo/bin/cargo install cargo-espflash ldproxy
RUN mkdir -p .espressif/frameworks/ \
    && git clone --branch ${ESP_IDF_VERSION} -q --depth 1 --shallow-submodules \
    --recursive https://github.com/espressif/esp-idf.git \
    .espressif/frameworks/esp-idf-v4.4 \
    && python3 .espressif/frameworks/esp-idf-v4.4/tools/idf_tools.py install cmake \
    && .espressif/frameworks/esp-idf-v4.4/install.sh ${ESP_BOARD} \
    && rm -rf .espressif/dist \
    && rm -rf .espressif/frameworks/esp-idf-v4.4/docs \
    && rm -rf .espressif/frameworks/esp-idf-v4.4/examples \
    && rm -rf .espressif/frameworks/esp-idf-v4.4/tools/esp_app_trace \
    && rm -rf .espressif/frameworks/esp-idf-v4.4/tools/test_idf_size
ENV IDF_TOOLS_PATH=/home/${CONTAINER_USER}/.espressif
