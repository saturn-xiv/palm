# LOQUAT

- Building

  ```bash
  $ mkdir .cache build
  $ chmod 777 .cache build
  $ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/mnt:z ubuntu:noble
  > cd /mnt/loquat/
  > ./build.sh
  ```

- Usage

  ```bash
  loquat -d rpc -p 11002
  ```
