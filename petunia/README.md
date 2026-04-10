# PETUNIA

## Usage

```bash
zig build -Dversion=$(date +"%Y.%-m.%-d")+$(git describe --tags --always --dirty --first-parent) -Doptimize=ReleaseSafe --summary all
```
