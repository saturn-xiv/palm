# Usage

```bash
# For Ubuntu
sudo apt install crun podman buildah fuse-overlayfs
# For ArchLinux
sudo pacman -S crun podman buildah fuse-overlayfs
```

- Decompress

  ```bash
  cat palm-spring-TIMESTAMP.tar.0? > palm-spring-TIMESTAMP.tar.xz
  ```

- Podman commands

  ```bash
  podman image prune # removes all dangling images
  podman system reset # clean
  
  podman load -i tmp/palm-CODE-TIMESTAMP.tar.xz # import image
  ```

- Merge file `~/.config/containers/storage.conf` and `~/.config/containers/registries.conf`
