# DAISY - RPC services for lavender

## Setup


## Usage

```bash
# Initial python3 virtual env
$ sudo apt install python3-full python3-dev build-essential libsystemd-dev
$ python3 -m venv $PWD/tmp/python

# Load virtual env vars
$ source $PWD/tmp/python/bin/activate

# Install dependencies for develpoment
> python3 -m pip install -e .

# Install for production
> python3 -m build
> pip install dist/daisy-0.1.0.post1.dev13+g46de0b167-py3-none-any.whl
> daisy -h
```

## Testing

```bash
# start rpc server
PYTHON_GIL=0 python3 -m daisy -d server -p 11006
PYTHON_GIL=0 python3 -m daisy -d worker
```

## Documents
