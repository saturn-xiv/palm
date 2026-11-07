# LOQUAT - A cryptographic rpc service(by Google Tink)

## Building

```bash
$ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/mnt:z ubuntu:noble
> cd /mnt/loquat/
> ./vcpkg/vcpkg depend-info --format tree
> ./build.sh
```

## Usage

```bash
./build/x86_64/loquat -d rpc -p 11001
```

## TODOS

- Upgrade [abseil-cpp-20260526](https://github.com/tink-crypto/tink-cc/blob/main/cmake/TinkWorkspace.cmake#L107C75-L107C94)
- Merge to BoringSsl
    - [We plan to release Tink 3.0, which will not support building Tink based on OpenSSL anymore](https://github.com/tink-crypto/tink-cc/issues/22)
    - [grpc/portfile.cmake](https://github.com/microsoft/vcpkg/blob/master/ports/grpc/portfile.cmake#L66)
