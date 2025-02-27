# LAVENDER

## Usage

### PostgreSQL

- Create a database

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE lavender WITH ENCODING = 'UTF8' OWNER www;
```

```bash
~/local/liquibase-4.29.2/liquibase --version
~/local/liquibase-4.29.2/liquibase --defaults-file=liquibase-postgresql.properties connect | status | rollback-count --count 1 | history
```

### Build

```bash
$ cd ~/workspace
$ ./saturn-xiv/palm/lavender/docker/start.sh
# build for amd64
> xmake f -p linux --toolchain=clang -a x86_64 -m release
> xmake
# build for aarch64
> xmake f -p linux --toolchain=clang -a arm64 -m release
> xmake f -p linux --toolchain=gcc -a arm64 -m release
> xmake
```
