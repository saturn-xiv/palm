# Building

```bash
$ podman run --rm -it --events-backend=file --network host -v $(dirname $PWD):/mnt:z ubuntu:noble
> cd /mnt/loquat/

> python3 -m venv $PWD/tmp/python
> source $PWD/tmp/python/bin/activate

> pip install cmake==3.31.10
> ./build.sh
```
