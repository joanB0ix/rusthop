# rusthop

A minimal URL‑shortening service written in Rust and laid out following the
hexagonal (ports & adapters) architecture.

## Why hexagonal?

* **Isolation of core logic** – `domain/` and `application/` are completely free
  from network, database, or framework code and can therefore be tested
  in‑memory and evolved independently.
* **Explicit boundaries** – interactions with the outside world happen only
  through *ports*.
* **Pluggable adapters** – you can add a Postgres repository or a different
  transport (gRPC, CLI, …) without touching the business rules.

## Building and running

```bash
# Build & run with the default in‑memory repository
cargo run

# Select a different listen address
cargo run -- --listen 127.0.0.1:3000
```

The default build enables the memory feature which includes an in‑process
HashMap‑based repository. To compile without it:

```bash
cargo build --no-default-features
```

## API

* **POST** `/`

    Request body:
    ```json
    { "url": "https://example.com", "ttl_secs": 600 }
    ```
    Response `201 Created`:
    ```json
    {
        "id": "A1b2C3d4",
        "original": "https://example.com",
        "created_at": "...",
        "expires_at": "...",
        "hits": 0
    }
    ```
* **GET** `/{id}`
    
    Redirects (`302`) to the original URL or returns `404` if the link does not exist or has expired.

## License

MIT — see `LICENSE` for details.