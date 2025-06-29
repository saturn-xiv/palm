# PALM

A total free education &amp; translation solution.

## Usage

```bash
$ ./docker/begonia/start.sh
> git clone https://github.com/saturn-xiv/palm.git /workspace/palm
> cd /workspace/palm
> git submodule update --init --recursive

> cmake --preset=x86_64
> cmake --build build/x86_64

> cmake --preset=aarch64
> cmake --build build/aarch64
```
