# Usage

```bash
# For Ubuntu
sudo apt install crun podman buildah fuse-overlayfs
# For ArchLinux
sudo pacman -S crun podman buildah fuse-overlayfs
```

- Decompress

```bash
cat palm-spring-TIMESTAMP.tar.0? > palm-spring-TIMESTAMP.tar
```

## Podman

```bash
podman image prune -a # removes all dangling images
podman system reset # clean
podman images # show images
podman ps -a # show containers
podman ps --filter status=running
podman logs xxxx
podman load -i tmp/palm-CODE-TIMESTAMP.tar # import image

# checking disk usage
podman system df
# cleaning up unused resources
podman system prune --all --volumes
```

- Merge file `~/.config/containers/storage.conf` and `~/.config/containers/registries.conf`

- Disable build cache `podman build --no-cache NAME`

- proxy settings `~/.config/containers/containers.conf`

```ini
[engine]
env=[
    "ALL_PROXY=socks5://127.0.0.1:8008",
    "HTTP_PROXY=socks5://127.0.0.1:8008",
    "HTTPS_PROXY=socks5://127.0.0.1:8008"
]
```

## Docker

- Merge file `/etc/docker/daemon.json`
