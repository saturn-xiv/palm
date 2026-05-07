# Usage

```bash
$ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/workspace:z ubuntu:jammy
> cd /workspace/loquat
> ./build.sh
> ls -lh build/Release/loquat
```
