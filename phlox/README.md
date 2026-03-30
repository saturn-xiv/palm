# PHLOX

## Testing

    ```bash
    cargo test --package phlox --test postgresql_test -- --nocapture
    cargo test --package phlox --test rabbitmq_test -- --nocapture --test consumer
    ```
