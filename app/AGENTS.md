# app/

Binary application services.

## STRUCTURE

```
app/
├── axum_server/         # HTTP REST API server
├── websocket_sockudo/   # WebSocket trading server
├── pingora_gateway/     # API gateway
├── xdp_libbpf/         # XDP network acceleration (eBPF)
└── client/              # Client examples
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| REST API | `app/axum_server/` |
| WebSocket | `app/websocket_sockudo/` |
| Gateway | `app/pingora_gateway/` |
| Network acceleration | `app/xdp_libbpf/` |

## CONVENTIONS

- Same as root
- Each has `main.rs` entry point

## ANTI-PATTERNS

- Same as root

## NOTES

- 6 binary crates
- Uses Axum for HTTP, custom XDP for network
