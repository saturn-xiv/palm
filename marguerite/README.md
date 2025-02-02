# MARGUERITE - NIFs for Thistle

## Usage

```bash
./marguerite/docker/start.sh
$ cd /workspace/marguerite/
$ ./build.sh
$ cd build/Release
$ erl
> c(marguerite).
> marguerite:version().
> marguerite:hmac_verify(marguerite:hmac_sign("Hi, Marguerite!"), "Hi, Marguerite!").
```
