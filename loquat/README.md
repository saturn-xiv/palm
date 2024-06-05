# USAGE

- Install toolchains

  ```bash
  $ cd ~/workspace/saturn-xiv/palm
  $ ./docker/start.sh
  > mkdir ~/build/loquat
  > cd ~/build/loquat
  > /workspace/loquat/setup.sh amd64 # OR arm64
  > xmake
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
