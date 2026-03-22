# Allow pkg-config to look for cross-compile libraries
export PKG_CONFIG_ALLOW_CROSS=1

# Tell pkg-config EXACTLY where the ARM libraries are
export PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
