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
- testing

  ```bash
  grpcurl -plaintext 127.0.0.1:10011 grpc.health.v1.Health/Check

  grpcurl -plaintext 127.0.0.1:10011 list
  grpcurl -plaintext 127.0.0.1:10011 list palm.portal.v1.Site
  grpcurl -plaintext 127.0.0.1:10011 palm.portal.v1.Site/Heartbeat

  # Using proto sources
  grpcurl -plaintext -import-path proto -proto portal.proto 127.0.0.1:10011 palm.portal.v1.Site/Heartbeat
  ```

## Documents

- [Version Support](https://protobuf.dev/support/version-support/)
- [Yocto Project Quick Build](https://docs.yoctoproject.org/brief-yoctoprojectqs/index.html)

## Issues

- vcpkg.json

```json
{ "name": "grpc", "features": ["codegen"] },
```
