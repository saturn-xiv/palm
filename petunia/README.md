# PETUNIA

## Usage

```bash
sudo apt install libssl-dev libcurl4-openssl-dev libpq-dev libmysqlclient-dev libsqlite3-dev libhiredis-dev librabbitmq-dev llibzlog-dev libgd-dev

# testing
zig build test --summary all

# build for release
zig build -Dversion=$(date +"%Y.%-m.%-d")+$(git describe --tags --always --dirty --first-parent) --release=safe --build-id=sha1 --summary all
```
