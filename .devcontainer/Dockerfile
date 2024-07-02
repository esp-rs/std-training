# Base image
FROM debian:bookworm-slim
ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8

# Arguments
ARG CONTAINER_USER=esp
ARG CONTAINER_GROUP=esp
ARG NIGHTLY_VERSION=nightly-2024-06-30
ARG ESP_IDF_VERSION=v5.2.2
ARG ESP_BOARD=esp32c3

# Install dependencies
RUN apt-get update \
    && apt-get install -y git wget flex bison gperf python3 python3-pip python3-venv \
    cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0 \
    llvm-dev libclang-dev clang pkg-config unzip libusb-1.0-0 \
    libpython3-all-dev python3-virtualenv curl \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Set users
RUN adduser --disabled-password --gecos "" ${CONTAINER_USER}
USER ${CONTAINER_USER}
WORKDIR /home/${CONTAINER_USER}

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain ${NIGHTLY_VERSION} -y --profile minimal \
    --component rust-src,clippy,rustfmt

# Update envs
ENV PATH=${PATH}:$HOME/.cargo/bin

# Install extra crates
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
    chmod u+x "${HOME}/.cargo/bin/ldproxy" && \
    GENERATE_VERSION=$(git ls-remote --refs --sort="version:refname" --tags "https://github.com/cargo-generate/cargo-generate" | cut -d/ -f3- | tail -n1) &&  \
    GENERATE_URL="https://github.com/cargo-generate/cargo-generate/releases/latest/download/cargo-generate-${GENERATE_VERSION}-${ARCH}.tar.gz" &&  \
    curl -L "${GENERATE_URL}" -o "${HOME}/.cargo/bin/cargo-generate.tar.gz" &&  \
    tar xf "${HOME}/.cargo/bin/cargo-generate.tar.gz" -C "${HOME}/.cargo/bin/" &&  \
    rm "${HOME}/.cargo/bin/cargo-generate.tar.gz" && \
    chmod u+x "${HOME}/.cargo/bin/cargo-generate"

# Install esp-idf
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

# Activate ESP environment
ENV IDF_TOOLS_PATH=${HOME}/.espressif
RUN echo "source ${HOME}/.espressif/frameworks/esp-idf/export.sh > /dev/null 2>&1" >> ~/.bashrc

CMD "/bin/bash"
