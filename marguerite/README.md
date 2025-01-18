# MARGUERITE - NIFs for Thistle

## Usage

```bash
$ ./build.sh
$ cd build/Release
$ erl
> c(marguerite).
> plain = fun() ->  end.
> code = marguerite:hmac_sign("Hi, Marguerite!").
> marguerite:hmac_verify(marguerite:hmac_sign("Hi, Marguerite!"), "Hi, Marguerite!").
```
