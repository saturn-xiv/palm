# PETUNIA

## Usage

```bash
zig build -Dversion=$(date +"%Y.%-m.%-d")+$(git describe --tags --always --dirty --first-parent) --release=safe --build-id=sha1 --summary all
```
