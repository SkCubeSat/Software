FROM debian:buster

# Install necessary tools for cross-compiling
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y wget libssl-dev && apt-get install -y pkg-config \
    apt-get install -y crossbuild-essential-armhf && \
    apt install -y openssh-server && \
    apt install -y gdb-multiarch && \
    apt install -y cmake && \
    apt install -y git && \
    apt-get install -y python3 python3-pip && \
    apt-get install -y can-utils && \
    apt-get update && apt-get install -y libsocketcan2 libsocketcan-dev && \
    apt-get install -y libzmq3-dev && \
    apt install -y tmux && \
    apt-get install -y kmod && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set the CMake version to install
ENV CMAKE_VERSION=3.28.3

# Download and install CMake from official website
RUN wget https://github.com/Kitware/CMake/releases/download/v${CMAKE_VERSION}/cmake-${CMAKE_VERSION}-linux-x86_64.sh -q -O /tmp/cmake-install.sh && \
    chmod u+x /tmp/cmake-install.sh && \
    /tmp/cmake-install.sh --skip-license --prefix=/usr/local && \
    rm /tmp/cmake-install.sh

# Verify installation
RUN cmake --version


# Set the working directory
WORKDIR /workspace

# Set the default command to bash
CMD ["bash"]

