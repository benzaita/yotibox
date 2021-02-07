FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1

RUN apt-get install -y make
RUN dpkg --add-architecture armhf \
 && apt-get update \
 && apt-get install -y -q libasound2-dev:armhf

RUN apt-get install -y pkg-config
ENV PKG_CONFIG=/usr/bin/arm-linux-gnueabihf-pkg-config
ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig