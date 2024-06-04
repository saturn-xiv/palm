# USAGE

- Install toolchains

  ```bash
  $ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/workspace:z ubuntu:noble
  > apt update
  > apt install -y build-essential xmake clang ccache \
      7zip curl git
  > apt install -y libsqlitecpp-dev libmysql++-dev libpqxx-dev libssl-dev python3-dev libboost-all-dev
  > xmake service --clean
  ```

- For x86_64

  ```bash
  xmake f -p linux -a x86_64 -m release --toolchain=clang
  ```

- For aarch64

  ```bash
  xmake f -p cross -m release --sdk=$HOME/local/arm-gnu-toolchain-13.2.Rel1-x86_64-aarch64-none-linux-gnu
  ```

## Documents

- [Arm GNU Toolchain Downloads](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
