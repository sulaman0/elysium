#  Changelog
This file contains changelogs that made in this specific branch

1. Fixed a panic issue that occurred during gasLimit overflow.
2. GasLimit now adjusts based on chain traffic (introducing a Fee Multiplier), allowing for values exceeding 6 million to 52 million (52_000_000)
3. Priority fee functionality has been implemented.
4. RPC node upgrades: Contract deployments and error messages are now more informative.
5. Introduced a MultiSig Authorities ratio.
6. Optimize transaction payload and adjust block weight to capture more transactions in a single block.
7. Upgrade chain code to v0.9.42
8. Introduce WeightV2 of POV in Blocks
9. Handle polkadot{.js} block details page requirements
10. Initiate POV size inside Blocks
11. Disable Elysium user generate coins function inside pallet >  elysium > lib.rs > generate_users_coins