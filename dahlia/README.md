# DAHLIA

## Usage

```bash
# initial python3 virtual env folder
$ python3 -m venv $HOME/tmp/python3

# load virtual env vars
$ source $HOME/tmp/python3/bin/activate
# install dependencies
> python3 -m pip install -e .

> python3 -m dahlia -h
```

## Testing

```bash
python3 -m dahlia -d
grpcurl -plaintext 127.0.0.1:8080 list
```

## Documents

- [Casbin Service](https://casbin.apache.org/docs/service/)
