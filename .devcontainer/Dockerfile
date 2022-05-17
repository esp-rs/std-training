# There are a few Dockerfile restrictions when using Github Actions
# See: https://docs.github.com/en/actions/creating-actions/dockerfile-support-for-github-actions

FROM debian:bullseye-slim
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ARG CONTAINER_USER=esp
ARG NIGHTLY_VERSION=nightly-2022-03-10
ARG ESP_IDF_VERSION=release/v4.4
ARG ESP_BOARD=esp32c3

RUN apt-get update \
    && apt-get install -y git curl ninja-build clang libudev-dev \
    python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

ENV PATH=${PATH}:$HOME/.cargo/bin

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain ${NIGHTLY_VERSION} -y --profile minimal \
    && $HOME/.cargo/bin/rustup component add rust-src --toolchain ${NIGHTLY_VERSION} \
    && $HOME/.cargo/bin/rustup target add riscv32i-unknown-none-elf

RUN $HOME/.cargo/bin/cargo install cargo-espflash espmonitor bindgen ldproxy

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
RUN echo "source /home/${CONTAINER_USER}/.espressif/frameworks/esp-idf-v4.4/export.sh > /dev/null 2>&1" >> ~/.bashrc

CMD "/bin/bash"
