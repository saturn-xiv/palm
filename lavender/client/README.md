# LAVENDER CLIENT

```bash
RUST_LOG=debug ./target/debug/wisteria -c wisteria/config.toml create-user-by-email -e "who-ami-i@gmail.com" -p "change-me"
RUST_LOG=debug ./target/debug/wisteria -c wisteria/config.toml list-user
RUST_LOG=debug ./target/debug/wisteria -c wisteria/config.toml add-role-for-user -u "xxx-xxx-xxx-xxx" -r "lavender.operator"
RUST_LOG=debug ./target/debug/wisteria -c wisteria/config.toml generate-token-for-email-user -e "who-ami-i@gmail.com" -w 1 -a "user.sign-in"

# start api server
RUST_LOG=debug,h2=info,lapin=info ./target/debug/wisteria -c wisteria/config.toml http -p 4000
# start email-send worker
RUST_LOG=debug,h2=info,lapin=info ./target/debug/wisteria -c wisteria/config.toml email-send-worker
# start lavender-job server
RUST_LOG=debug,h2=info,lapin=info ./target/debug/wisteria -c wisteria/config.toml lavender-job-worker

./lavender.sh version
```
