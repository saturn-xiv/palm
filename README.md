# PALM

A total free education &amp; translation solution.

## Usage

```bash
$ ./docker/begonia/start.sh
> git clone https://github.com/saturn-xiv/palm.git /workspace/palm
> cd /workspace/palm
> git submodule update --init --recursive

> cmake --preset=default -DVCPKG_TARGET_TRIPLET=x64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$PWD/toolchains/clang/x86_64.cmake
> cmake --build

> cmake --preset=default -DVCPKG_TARGET_TRIPLET=arm64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$PWD/toolchains/clang/aarch64.cmake
> cmake --build
```
