# USAGE

```bash
$ podman run --rm -it --events-backend=file --hostname=palm --network host -v $(dirname $PWD):/workspace:z ubuntu:latest
> cd /workspace/loquat
> ./build.sh
```
