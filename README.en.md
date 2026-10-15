# BikeStationAPI

A small Axum API built to explore tracing, timeouts and rate limiting, without focusing on complex business logic.

The domain — a bike-share network exposing station availability — is deliberately thin. The point of the project is everything around the endpoints: structured logging, request timeouts, per-client rate limiting, a background refresh task, and graceful shutdown.

## Endpoints

| Method | Path | Description | Responses |
|--------|------|-------------|-----------|
| GET | `/stations` | List all stations | `200` JSON array |
| GET | `/stations/{id}` | One station by id | `200` JSON, `404` if unknown |
| POST | `/stations/{id}/report` | Report a broken bike (decrements available bikes by one) | `200` updated station, `404` if unknown, `409` if none available |
| GET | `/health` | Liveness probe | `200` |

A station is `{ id, name, total_docks, available_bikes, free_docks }`. State is kept in memory (no database) and seeded with four stations at startup.

## How it works

Every request goes through three tower layers, from outermost to innermost:

1. **Tracing** (`tower-http` `TraceLayer`) — logs method, path, status and latency for every request, including the ones rejected below.
2. **Rate limiting** (`axum-governor`) — per client IP, 50 requests/second. Over the limit returns `429` with `RateLimit` and `Retry-After` headers.
3. **Timeout** (`tower-http` `TimeoutLayer`) — 5s per request; exceeding it returns `408`.

The order is intentional: tracing is outermost so rejected requests still show up in the logs, and rate limiting runs before the timeout so abusive traffic is dropped as early as possible.

Alongside the server, a background task refreshes the station state every 5s from a simulated source (a function with artificial latency). That call is bounded by its own 2s timeout; if it is exceeded, the refresh is skipped and the task keeps running. The refresh replaces the in-memory state, so a `report` is reflected only until the next refresh.

On `Ctrl-C`, the server stops accepting connections and the background task is cancelled; the process waits for both to finish before exiting.

## Running

Requires a stable Rust toolchain (edition 2024, 1.85+).

```
cargo run
```

The server listens on `127.0.0.1:3000`.

```
curl -i http://127.0.0.1:3000/stations
curl -i http://127.0.0.1:3000/stations/1
curl -i -X POST http://127.0.0.1:3000/stations/1/report
```

## Configuration

Log level is controlled by `RUST_LOG` (defaults to `info` when unset):

```
RUST_LOG=debug cargo run
```

HTTP access logs (the tracing layer) are emitted at `DEBUG`, so the default `info` level shows only application events: startup, background-refresh timeouts, shutdown. Use `RUST_LOG=debug` or `RUST_LOG=info,tower_http=debug` to see per-request logs.

## Tests

```
cargo test
```

Covers the endpoint behaviours (status codes and JSON), the background refresh and its timeout, rate-limit admission and rejection including per-client isolation, and graceful shutdown of the background task.

## Project layout

```
src/
  main.rs        server startup, tracing init, background task, graceful shutdown
  routes.rs      router and the tower layer stack
  state.rs       shared in-memory state (Arc<RwLock<Vec<Station>>>)
  models.rs      Station model and seed data
  handlers/      endpoint handlers
  sources.rs     simulated source + background refresh task
  shutdown.rs    Ctrl-C signal
tests/           integration tests
```

## Notes

- CI (GitHub Actions) runs `fmt`, `clippy -D warnings`, `check`, `test` and a release build on every push and pull request to `main`.
- `Cargo.toml` carries some auth-related dependencies (`argon2`, `jsonwebtoken`, `dotenvy`) that are not used yet — JWT auth is a possible next step, not part of the current code.
- No license file is included.