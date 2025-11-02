# USAGE

```bash
# https://devguide.python.org/versions/
$ ./docker/start.sh
> cd ~/build/
> export LIBRARY_PATH=$PWD/build/deps/lib:$PWD/build/deps/lib64:$LIBRARY_PATH

> portable-python build-report 3.13.2 -m libffi,zlib,xz,bzip2,openssl,uuid,sqlite,readline,bdb,readline
> portable-python build 3.13.2 -m libffi,zlib,xz,bzip2,openssl,uuid,sqlite,readline,bdb,readline
> tar -xf dist/cpython-3.13.2-linux-x86_64.tar.gz -C ~/local/
> ~/local/3.13.2/bin/python -m venv ~/local/python3.13
> . ~/local/python3.13/bin/activate
> pip install --upgrade pip
> pip install portable-python
> portable-python inspect ~/local/3.13.2/bin/python
> python3 -c 'import ssl; print(ssl.OPENSSL_VERSION)'
> python3 -c 'import sqlite3; print(sqlite3.sqlite_version)'
> deactivate

# https://wiki.python.org/moin/BuildStatically
> LDFLAGS="-Wl,-rpath,/usr/local/lib -Wl,-rpath,/usr/local/lib64" CONFIGURE_OPTS="--disable-shared --enable-optimizations --with-lto --with-openssl=/usr/local --with-openssl-rpath=auto" pyenv install -v 3.13.2
```

| OS            | Glibc version |                                     |
| ------------- | ------------- | ----------------------------------- |
| centos v7.4   | 2.17          | CentOS-7-x86_64-Everything-2003.iso |
| kylin v10     | 2.31          |                                     |
| ubuntu focal  | 2.31          |                                     |
| ubuntu xenial | 2.23          |                                     |
| ubuntu trusty | 2.19          |                                     |
| Amazon Linux  | 2.17          | 2016.09.0.20161028                  |

- [Portable python binaries](https://github.com/codrsquad/portable-python)
