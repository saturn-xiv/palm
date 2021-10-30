# Cross compile by clang toolchain

## Usage

```bash
sudo apt install podman buildah runc lxc
# clear all images & containers
podman rmi -a -f
# import new images
podman load -q -i palm-clang-TIMESTAMP.tar
```

## Documents

- [How to cross compile with LLVM based tools](https://archive.fosdem.org/2018/schedule/event/crosscompile/attachments/slides/2107/export/events/attachments/crosscompile/slides/2107/How_to_cross_compile_with_LLVM_based_tools.pdf)
- [Assembling a Complete Toolchain](https://clang.llvm.org/docs/Toolchain.html)
- [Toolchains](https://elinux.org/Toolchains)
