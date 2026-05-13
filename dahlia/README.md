# DAHLIA

## Usage

```bash
# Initial python3 virtual env
$ sudo apt install python3-full python3-dev build-essential
$ python -m venv $PWD/tmp/python

# Load virtual env vars
$ source $PWD/tmp/python/bin/activate
# Install dependencies
> python -m pip install -e .

> dahlia -h
```

## Testing

```bash
python -m dahlia -d
grpcurl -plaintext 127.0.0.1:8080 list

python -m unittest tests.py
```

## Documents

- [Casbin Service](https://casbin.apache.org/docs/service/)
