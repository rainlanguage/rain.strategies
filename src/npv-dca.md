This is a variant of the auction-dca strategy. In this strategy, the floor price for the auction is calculated as the "fair value" of the token based on present value of future cash flows. The production profiles and discount rates are hardcoded. The fair value will vary based on:
- Benchmark oil prices, which are fetched from pyth oracle
- The current time (as time progresses more of the production gets discarded as already historical)
- Token supply, if new mints occur