# TULIP

## Usage

```bash
$ ./tulip/docker/start.sh
> cd /workspace/tulip/

> cmake --preset=x86_64
> cmake --build build/x86_64
> cmake --preset=aarch64
> cmake --build build/aarch64

> xmake f --toolchain=clang --runtimes=stdc++_static -m release
> xmake f --toolchain=gcc --runtimes=stdc++_static -m release
> xmake -y
```

## Documents

- [Version Support](https://protobuf.dev/support/version-support/)
