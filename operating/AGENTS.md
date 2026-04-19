# proc/operating/

Business process orchestration for exchange operations.

## STRUCTURE

```
proc/operating/
├── exchange/
│   ├── spot/              # Spot trading process
│   ├── derivatives/       # Derivatives trading
│   ├── wallet/            # Wallet management
│   └── spot_market_data/  # Market data
└── push/push/             # Notification push
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Spot trading | `proc/operating/exchange/spot/` |
| Derivatives | `proc/operating/exchange/derivatives/` |
| Wallet | `proc/operating/exchange/wallet/` |

## CONVENTIONS

- Same as root
- Process crates use `proc/v2/actor` pattern

## ANTI-PATTERNS

- Same as root

## NOTES

- 5 business process crates
- Uses DDD layer separation via crate boundaries
