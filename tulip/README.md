# TULIP

## Usage

```bash
$ ./tulip/docker/run.sh
> cd /workspace/tulip/
> ./scripts/build.sh
```

## Development

- start backend `./build/x86_64/tulip -d rpc -p 10011`
- start frontend `npm run dev`
- start proxy `envoy --use-dynamic-base-id -l debug -c envoy.yaml`
- open browser `http://localhost:4000`

## Documents

- [Version Support](https://protobuf.dev/support/version-support/)
- [Yocto Project Quick Build](https://docs.yoctoproject.org/brief-yoctoprojectqs/index.html)
