# Usage

```bash
$ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/workspace:z ubuntu:latest
> cd /workspace
> ./scripts/go.sh jasmine
> ls -lh tmp/
```
