# LAVENDER - Aggregating your logs, tracking the health, uptime, and performance of your hardware and networks

```bash
# generate job sample
cargo test -p lavender job_sample -- --exact --show-output

# push job
./lavender.sh echo 'Palm!'

# execute job
RUST_LOG=debug,h2=info,lapin=info ./target/debug/wisteria -c wisteria/config.toml lavender-job-worker -i 10000
RUST_LOG=debug,h2=info,lapin=info ./target/debug/wisteria -c wisteria/config.toml email-send-worker -i 60000
```
