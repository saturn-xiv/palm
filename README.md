# PALM - A total free education & translation solution

## Deployment

```bash
$ podman run --rm -it --events-backend=file --network host -v $PWD:/workspace:z ubuntu:latest
> cd /workspace/
> mkdir -p ~/downloads/
> cp dm-go-driver.zip ~/downloads/
> ./scripts/build.sh
```

## Documents

- [gRPC over HTTP2](https://grpc.github.io/grpc/core/md_doc__p_r_o_t_o_c_o_l-_h_t_t_p2.html)
