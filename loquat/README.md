# Usage

## Testing

```bash
curl -v -XPOST -d '{"query": "{ version }"}' http://127.0.0.1:8080/graphql
curl -v -XPOST -d '{"query": "mutation { signOut{createdAt} }"}' http://127.0.0.1:8080/graphql
```

## Timer

```bash
systemctl list-timers --all
```
