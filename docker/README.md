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

- Podman commands

  ```bash
  podman image prune # removes all dangling images
  podman system reset # clean
  podman images # show images
  podman ps -a # show containers
  podman ps --filter status=running
  podman logs xxxx
  podman load -i tmp/palm-CODE-TIMESTAMP.tar # import image
  ```

- Merge file `~/.config/containers/storage.conf` and `~/.config/containers/registries.conf`

- Disable build cache `podman build --no-cache NAME`
