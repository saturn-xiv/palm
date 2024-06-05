# USAGE

```bash
$ ~/workspace/saturn-xiv/palm-cpp/docker/start.sh
> mkdir -p $HOME/build/palm
> cd $HOME/build/palm
> xmake f -p linux -a x86_64 -m release --toolchain=clang -P /workspace/saturn-xiv/palm-cpp/loquat
> xmake f -p cross -m release --sdk=$HOME/local/arm-gnu-toolchain-13.2.Rel1-x86_64-aarch64-none-linux-gnu -P /workspace/saturn-xiv/palm-cpp/loquat
```
