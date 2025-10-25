# USAGE

```bash
go install github.com/asdf-vm/asdf/cmd/asdf@v0.18.0
echo 'export PATH="${ASDF_DATA_DIR:-$HOME/.asdf}/shims:$PATH"' >> ~/.zshrc

asdf plugin list
asdf plugin list all
asdf plugin update --all

asdf plugin add erlang https://github.com/asdf-vm/asdf-erlang.git
asdf install erlang 28.1.1
asdf set erlang 28.1.1
erl --version

asdf plugin add elixir https://github.com/asdf-vm/asdf-elixir.git
asdf install elixir 1.19.1
asdf set elixir 1.19.1
elixir --version

asdf plugin add nodejs https://github.com/asdf-vm/asdf-nodejs.git
asdf install nodejs 22.21.0
asdf set nodejs 22.21.0
node -v
npm -v
```

## Documents

- [Elixir Version Managers](https://elixir-lang.org/install.html#version-managers)
- [Phoenix installation](https://hexdocs.pm/phoenix/installation.html)
