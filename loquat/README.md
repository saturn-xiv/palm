# LOQUAT - A cryptographic rpc service(by Google Tink)

- Building

  ```bash
  $ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/mnt:z ubuntu:noble
  > cd /mnt/loquat/
  > ./build.sh
  ```

- Usage

  ```bash
  ./build/x86_64/loquat -d rpc -p 11001
  ```
