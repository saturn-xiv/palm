# Usage

```bash
$ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/mnt:z ubuntu:noble
> cd /mnt/loquat/
> ./build.sh
```
