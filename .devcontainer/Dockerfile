# There are a few Dockerfile restrictions when using Github Actions
# See: https://docs.github.com/en/actions/creating-actions/dockerfile-support-for-github-actions

FROM debian:bullseye-slim
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ARG NIGHTLY_VERSION=nightly-2023-02-28
ARG ESP_IDF_VERSION=v4.4.1
ARG ESP_BOARD=esp32c3

RUN apt-get update \
    && apt-get install -y git curl ninja-build llvm-dev libclang-dev \
    clang unzip python3 python3-pip pkg-config \
    libssl-dev \ 
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

ENV PATH=${PATH}:$HOME/.cargo/bin

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain ${NIGHTLY_VERSION} -y --profile minimal \
    --component rust-src,clippy

RUN $HOME/.cargo/bin/cargo install cargo-generate

RUN ARCH=$($HOME/.cargo/bin/rustup show | grep "Default host" | sed -e 's/.* //') && \
    curl -L "https://github.com/esp-rs/espflash/releases/latest/download/cargo-espflash-${ARCH}.zip" -o "${HOME}/.cargo/bin/cargo-espflash.zip" && \
    unzip "${HOME}/.cargo/bin/cargo-espflash.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/cargo-espflash.zip" && \
    chmod u+x "${HOME}/.cargo/bin/cargo-espflash" && \
    curl -L "https://github.com/esp-rs/espflash/releases/latest/download/espflash-${ARCH}.zip" -o "${HOME}/.cargo/bin/espflash.zip" && \
    unzip "${HOME}/.cargo/bin/espflash.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/espflash.zip" && \
    chmod u+x "${HOME}/.cargo/bin/espflash" && \
    curl -L "https://github.com/esp-rs/esp-web-flash-server/releases/latest/download/web-flash-${ARCH}.zip" -o "${HOME}/.cargo/bin/web-flash.zip" && \
    unzip "${HOME}/.cargo/bin/web-flash.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/web-flash.zip" && \
    chmod u+x "${HOME}/.cargo/bin/web-flash" && \
    curl -L "https://github.com/esp-rs/embuild/releases/latest/download/ldproxy-${ARCH}.zip" -o "${HOME}/.cargo/bin/ldproxy.zip" &&  \
    unzip "${HOME}/.cargo/bin/ldproxy.zip" -d "${HOME}/.cargo/bin/" && \
    rm "${HOME}/.cargo/bin/ldproxy.zip" && \
    chmod u+x "${HOME}/.cargo/bin/ldproxy"



RUN mkdir -p ${HOME}/.espressif/frameworks/ \
    && git clone --branch ${ESP_IDF_VERSION} -q --depth 1 --shallow-submodules \
    --recursive https://github.com/espressif/esp-idf.git \
    ${HOME}/.espressif/frameworks/esp-idf \
    && python3 ${HOME}/.espressif/frameworks/esp-idf/tools/idf_tools.py install cmake \
    && ${HOME}/.espressif/frameworks/esp-idf/install.sh ${ESP_BOARD} \
    && rm -rf .espressif/dist \
    && rm -rf .espressif/frameworks/esp-idf/docs \
    && rm -rf .espressif/frameworks/esp-idf/examples \
    && rm -rf .espressif/frameworks/esp-idf/tools/esp_app_trace \
    && rm -rf .espressif/frameworks/esp-idf/tools/test_idf_size
ENV IDF_TOOLS_PATH=${HOME}/.espressif
RUN echo "source ${HOME}/.espressif/frameworks/esp-idf/export.sh > /dev/null 2>&1" >> ~/.bashrc
CMD "/bin/bash"
