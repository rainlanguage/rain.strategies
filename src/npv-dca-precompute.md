A gas-optimized variant of npv-dca. Instead of calculating 32 discount factors at runtime, this version pre-computes the discounted cumulative future production for each month.

At runtime, only the commodity price (from Pyth) and current time vary. The current time determines which month we're in, pro-rates the current month's production, and calculates the single discount factor from now to end of month. This reduces the calculation to a pro-rated term for the current month and a terminal value representing all the future whole months.

This reduces the NPV calculation from 32 discount factor operations to just 1, while remaining mathematically exact.
