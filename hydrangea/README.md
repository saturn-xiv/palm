# HYDRANGEA - Aggregating your logs, tracking the health, uptime, and performance of your hardware and networks

## Setup

## Usage

```bash
# Initial python3 virtual env
$ sudo apt install python3-full python3-dev build-essential libsystemd-dev
$ python3 -m venv $PWD/tmp/python

# Load virtual env vars
$ source $PWD/tmp/python/bin/activate

# Install dependencies for development
> python3 -m pip install -e .

# Install for production
> python3 -m build
> pip install dist/hydrangea-0.1.0.post1.dev169+g37e9387f1.d20261106-py3-none-any.whl
> hydrangea -h
```

## Testing

```bash
# start rpc server
python3 -m hydrangea -h

PYTHON_GIL=0 python3 -m hydrangea -d worker -w 2
sudo -E env PATH="$PATH" PYTHON_GIL=0 python3 -m hydrangea -d worker -w 2
```

## Documents
