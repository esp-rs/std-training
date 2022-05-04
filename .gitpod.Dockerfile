# Note: gitpod/workspace-base image references older version of CMake, it's necessary to install newer one
FROM  gitpod/workspace-base
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ARG CONTAINER_USER=gitpod
ARG CONTAINER_GROUP=gitpod
RUN sudo install-packages git curl gcc ninja-build libudev-dev \
  python3 python3-pip libusb-1.0-0 libssl-dev pkg-config libtinfo5 clang \
    && pip3 install websockets==10.2
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}
ARG NIGHTLY_VERSION=nightly-2022-03-10
ENV PATH=${PATH}:/home/${CONTAINER_USER}/.cargo/bin:/home/${CONTAINER_USER}/opt/bin
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain ${NIGHTLY_VERSION} -y \
    && $HOME/.cargo/bin/rustup component add rust-src --toolchain ${NIGHTLY_VERSION} \
    && $HOME/.cargo/bin/rustup target add riscv32i-unknown-none-elf \
    && $HOME/.cargo/bin/cargo install cargo-generate cargo-espflash espmonitor bindgen ldproxy
ADD --chown=${CONTAINER_USER}:${CONTAINER_GROUP} \
  https://github.com/Kitware/CMake/releases/download/v3.23.1/cmake-3.23.1-linux-x86_64.sh \
  /home/${CONTAINER_USER}/cmake-install.sh
RUN chmod a+x /home/gitpod/cmake-install.sh \
  && mkdir -p /home/gitpod/opt \
    && ./cmake-install.sh --prefix=/home/gitpod/opt --skip-license
ENV ESP_BOARD=esp32c3
ENV ESP_IDF_VERSION=release/v4.4
RUN mkdir -p /home/${CONTAINER_USER}/.espressif/frameworks/ \
    && git clone -b ${ESP_IDF_VERSION} --depth 1 --shallow-submodules \
    --recursive https://github.com/espressif/esp-idf.git \
    /home/${CONTAINER_USER}/.espressif/frameworks/esp-idf-v4.4 \
    && /home/${CONTAINER_USER}/.espressif/frameworks/esp-idf-v4.4/install.sh ${ESP_BOARD} \
    && rm -rf /home/${CONTAINER_USER}/.espressif/dist \
    && rm -rf /home/${CONTAINER_USER}/.espressif/frameworks/esp-idf-v4.4/docs