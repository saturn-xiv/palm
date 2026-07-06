# Usage

## Building

```bash
cmake -Wno-dev -DCMAKE_BUILD_TYPE=Release -DABSL_PROPAGATE_CXX_STD=ON -DTINK_USE_SYSTEM_OPENSSL=ON -DTINK_BUILD_TESTS=OFF -B build -S . -G Ninja
cmake --build build
```

## Testing

**Note** Change hostname to `palm.change-me.prg` and add domain to `/etc/hosts` before testing.

- start c-node

```bash
./build/loquat -d c-node  -c "change-me" -n loquat@localhost -p 9092
```

- start

```bash
$ erl -name demo@palm.change-me.org -setcookie "change-me"
> register(my_demo_server, self()).

> registered().
> whereis(my_demo_server).

> global:registered_names().
> global:whereis_name(GlobalName).
> erlang:process_info(Pid, registered_name).

> c(loquat).
> loquat:foo(3).
```
