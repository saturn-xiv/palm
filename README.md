# PALM

A total free education &amp; translation solution.

## Usage

```bash
$ ./docker/begonia/start.sh
> git clone https://github.com/saturn-xiv/palm.git /workspace/palm
> cd /workspace/palm
> git submodule update --init --recursive

# FIXME https://github.com/gabime/spdlog/issues/3306
> cmake --preset=default -DVCPKG_TARGET_TRIPLET=x64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$PWD/toolchains/clang/x86_64.cmake \

> cmake --build

> cmake --preset=default -DVCPKG_TARGET_TRIPLET=arm64-linux-release -DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=$PWD/toolchains/gcc/aarch64.cmake \
    -DCMAKE_BUILD_TYPE=Release -DBUILD_COMPILER=OFF -DWITH_OPENSSL=ON -DBUILD_JAVA=OFF -DBUILD_JAVASCRIPT=OFF -DBUILD_NODEJS=OFF -DBUILD_PYTHON=OFF
> cmake --build
```

## Documents

- [gRPC for Web Clients](https://github.com/grpc/grpc-web)
- [RBAC96](https://profsandhu.com/cs6393_s12/lecture-rbac96.pdf)
- [AppImage Building Tool](https://github.com/linuxdeploy/linuxdeploy/releases/)
- [OpenSearch Data types](https://docs.opensearch.org/latest/search-plugins/sql/datatypes/)
- [gRPC Status Codes](https://grpc.io/docs/guides/status-codes/)
- [ProtoJSON Format](https://protobuf.dev/programming-guides/json/)
- [Protocol Buffers Version Support](https://protobuf.dev/support/version-support/)
- [Maven Quickstart Archetype](https://maven.apache.org/archetypes/maven-archetype-quickstart/)
- [WeUI - tailor-made for WeChat web service](https://github.com/Tencent/weui)
