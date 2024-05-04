# USAGE

```bash
$ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/workspace:z ubuntu:latest
> ./build.sh
> node dist/morus.xxx.bundle.js config.json
```
