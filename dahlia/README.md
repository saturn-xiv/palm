# DAHLIA

## Usage

```bash
# Initial python3 virtual env
$ python3 -m venv $HOME/tmp/python3

# Load virtual env vars
$ source $HOME/tmp/python3/bin/activate
# Install dependencies
> python3 -m pip install -e .

> python3 -m dahlia -h
```

## Testing

```bash
python3 -m dahlia -d
grpcurl -plaintext 127.0.0.1:8080 list

python -m unittest tests.py
```

## Documents

- [Casbin Service](https://casbin.apache.org/docs/service/)
