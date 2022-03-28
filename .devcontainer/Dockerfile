ARG VARIANT=bullseye
FROM debian:${VARIANT}
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
RUN apt-get update \
    && apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev \
     python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts
RUN adduser --disabled-password --gecos "" vscode
USER vscode
WORKDIR /home/vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly-2022-03-10 -y
ENV PATH=${PATH}:$HOME/.cargo/bin:$HOME/.cargo/bin
RUN $HOME/.cargo/bin/rustup component add rust-src --toolchain nightly-2022-03-10
RUN $HOME/.cargo/bin/rustup target add riscv32i-unknown-none-elf
RUN $HOME/.cargo/bin/cargo install cargo-generate cargo-espflash espmonitor bindgen ldproxy
ENV USER=vscode
ENV ESP_IDF_TOOLS_INSTALL_DIR=global
ARG ESP_IDF_VER
ARG ESP_IDF_BRANCH
ENV ESP_IDF_VER=${ESP_IDF_VER}
ENV ESP_IDF_VERSION=${ESP_IDF_BRANCH}
ENV ESP_IDF_BRANCH=${ESP_IDF_BRANCH}
RUN git clone --recursive --depth 1 --shallow-submodules -b ${ESP_IDF_BRANCH} \
    https://github.com/espressif/esp-idf.git /home/vscode/esp-idf
ARG ESP_BOARD
ENV ESP_BOARD=${ESP_BOARD}
RUN /home/vscode/esp-idf/install.sh ${ESP_BOARD}
ENV IDF_PATH=/home/vscode/esp-idf
RUN echo source /home/vscode/esp-idf/export.sh >> ~/.bashrc
RUN echo export IDF_PYTHON_ENV_PATH=/home/vscode/.local/bin >> ~/.bashrc
