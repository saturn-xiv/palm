# PALM

A total free education &amp; translation solution.

## Usage

```bash
$ ./docker/begonia/start.sh
> git clone https://github.com/saturn-xiv/palm.git /workspace/palm
> cd /workspace/palm
> git submodule update --init --recursive

> cmake --preset=default -DVCPKG_TARGET_TRIPLET=x64-linux-release
> cmake --build

> cmake --preset=default -DVCPKG_TARGET_TRIPLET=arm64-linux-release
> cmake --build
```
