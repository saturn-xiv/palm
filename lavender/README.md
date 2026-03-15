# LAVENDER - A logging crawler

```bash
./scripts/build.sh
```

## Issues

- Too many open files

  ```bash
  sysctl -w fs.inotify.max_user_watches=512000
  ```
