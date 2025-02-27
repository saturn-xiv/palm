# LAVENDER

## Usage

```bash
$ cd ~/workspace
$ ./saturn-xiv/palm/lavender/docker/start.sh
# build for amd64
> xmake f -p linux --toolchain=clang -a x86_64 -m release
> xmake
# build for aarch64
> xmake f -p linux --toolchain=clang -a arm64 -m release
> xmake f -p linux --toolchain=gcc -a arm64 -m release
> xmake
```
