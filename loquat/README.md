# Usage

```bash
$ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/workspace:z ubuntu:noble
> cd /workspace/loquat/
> ./build.sh
```
