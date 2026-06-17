FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# Install core build dependencies needed by cross
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    cmake \
    ninja-build \
    wget \
    python3 \
    && rm -rf /var/lib/apt/lists/*

# Install LLVM-MinGW for the windows-gnullvm toolchain
WORKDIR /opt
RUN wget https://github.com/mstorsjo/llvm-mingw/releases/download/20240606/llvm-mingw-20240606-ucrt-ubuntu-20.04-x86_64.tar.xz && \
    tar -xf llvm-mingw-20240606-ucrt-ubuntu-20.04-x86_64.tar.xz && \
    rm llvm-mingw-20240606-ucrt-ubuntu-20.04-x86_64.tar.xz && \
    mv llvm-mingw-20240606-ucrt-ubuntu-20.04-x86_64 llvm-mingw

# Expose binaries to the container path
ENV PATH="/opt/llvm-mingw/bin:${PATH}"

# Configure the exact cross-compilation environment variables cross needs
ENV CARGO_TARGET_AARCH64_PC_WINDOWS_GNULLVM_LINKER="aarch64-w64-mingw32-clang" \
    CC_aarch64_pc_windows_gnullvm="aarch64-w64-mingw32-clang" \
    CXX_aarch64_pc_windows_gnullvm="aarch64-w64-mingw32-clang++" \
    AR_aarch64_pc_windows_gnullvm="aarch64-w64-mingw32-ar"
