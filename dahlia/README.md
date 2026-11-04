# DAHLIA - RBAC services

## Setup

- PostgreSql

  ```sql
  CREATE DATABASE dahlia_dev WITH OWNER www;
  ```

- RabbitMq

  ```bash
  sudo rabbitmqctl add_vhost dahlia.dev
  sudo rabbitmqctl set_permissions -p dahlia.dev www ".*" ".*" ".*"
  ```

## Usage

```bash
# Initial python3 virtual env
$ sudo apt install python3-full python3-dev build-essential
$ python3 -m venv $PWD/tmp/python

# Load virtual env vars
$ source $PWD/tmp/python/bin/activate

# Install dependencies for develpoment
> python3 -m pip install -e .
# start rpc server
> python3 -m dahlia -d -p 11002

# Install for production
> python3 -m build
> pip install dist/dahlia-0.1.0.post1.dev13+g46de0b167-py3-none-any.whl
> dahlia -h
```

## Testing

```bash
# test by grpcurl
grpcurl -plaintext 127.0.0.1:11002 list

# run unittests
python3 -m unittest tests.py
```

## Documents

- [Casbin Service](https://casbin.apache.org/docs/service/)
