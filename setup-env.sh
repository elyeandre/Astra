export CHOST="armv7l-linux-musleabihf"
export TOOLCHAIN="$HOME/armv7l-linux-musleabihf-cross/bin"
export PATH="$TOOLCHAIN:$PATH"

export CC="$TOOLCHAIN/$CHOST-gcc"
export CXX="$TOOLCHAIN/$CHOST-g++"
export AR="$TOOLCHAIN/$CHOST-ar"
export AS="$TOOLCHAIN/$CHOST-as"
export LD="$TOOLCHAIN/$CHOST-ld"
export STRIP="$TOOLCHAIN/$CHOST-strip"
export RANLIB="$TOOLCHAIN/$CHOST-ranlib"

export CARGO_BUILD_JOBS=$(nproc)
