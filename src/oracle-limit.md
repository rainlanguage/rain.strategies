# Oracle Limit

A limit order that uses a signed context oracle for pricing instead of a hardcoded exchange rate.

## How it works

1. An oracle server fetches a price feed (e.g. ETH/USD from Pyth) and signs it as `SignedContextV1`
2. The oracle URL is encoded in the order's onchain metadata via `SignedContextOracleV1`
3. When taking or clearing the order, the caller fetches signed context from the oracle and includes it in the transaction
4. The order's rainlang reads the oracle price from `signed-context<0 0>()` and the expiry from `signed-context<0 1>()`
5. The order verifies the data hasn't expired, applies an optional spread, and sets the IO ratio

## Parameters

| Parameter | Description |
|-----------|-------------|
| `oracle-spread` | Percentage above oracle price to sell at (e.g. 0.01 = 1%). Set to 0 to sell at exact oracle price. |

## Oracle server

The order expects an oracle server at the URL specified in `oracle-url` that responds to `GET /context` with:

```json
{
  "signer": "0x...",
  "context": ["0x...<price_18_decimals>", "0x...<expiry_timestamp>"],
  "signature": "0x..."
}
```

See [rain-oracle-server](https://github.com/rainlanguage/rain-oracle-server) for a reference implementation.

## Networks

- Base
