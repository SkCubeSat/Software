FROM debian:buster

# Install necessary tools for cross-compiling
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y crossbuild-essential-armhf && \
    apt install -y openssh-server && \
    apt install -y gdb-multiarch && \
    apt install -y cmake && \
    apt install -y git

# Set the working directory
WORKDIR /workspace

# Set the default command to bash
CMD ["bash"]

