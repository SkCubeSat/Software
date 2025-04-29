FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

# Install core development tools
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    python3 \
    python3-pip \
    python3-venv \
    curl \
    git \
    unzip \
    pkg-config \
    libusb-1.0-0-dev \
    libssl-dev \
    libffi-dev \
    gcc \
    g++ \
    vim \
    wget \
    && apt-get clean

# Install Rust (for Rust-based services)
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Upgrade pip and install pip-tools (for Python dependency management)
RUN pip3 install --upgrade pip setuptools pip-tools

# Optional: Add your preferred KubOS tools or dependencies here

# Default workdir - mount your code here
WORKDIR /workspace

CMD ["/bin/bash"]
