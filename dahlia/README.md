# DAHLIA

## Setup

- PostgreSql

  ```sql
  CREATE USER www WITH PASSWORD 'change-me';
  CREATE DATABASE dahlia_dev WITH OWNER www;
  ```

- RabbitMq

  ```bash
  sudo rabbitmqctl add_user www "change-me"
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
# Install dependencies
> python -m pip install -e .
# Install for production
> python -m pip install .

> dahlia -h
```

## Testing

```bash
PYTHON_GIL=0 python -m dahlia -d -p 11001
grpcurl -plaintext 127.0.0.1:8080 list

PYTHON_GIL=0 python -m unittest tests.py
```

## Documents

- [Casbin Service](https://casbin.apache.org/docs/service/)
