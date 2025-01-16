# Security policy

## Supported versions

Currently, only the latest master commit pin is supported. This will be extended back to releases as soon as we fix the Substrate release pipeline.

## Reporting vulnerabilities

For medium or high severity security vulnerabilities, please report them by email to security@parity.io. If you think your report might be eligible for the Parity Bug Bounty Program, your email should be sent to bugbounty@parity.io. Please make sure to follow [guidelines](https://www.parity.io/bug-bounty/) when reporting.

For low severity security vulnerabilities, you can either follow the above reporting pipeline or open an issue in the Frontier repo directly. If you are unsure about the severity of the vulnerability you're reporting, please reach out to [Wei](mailto:wei@parity.io).

## Advisory announcements

Due to the nature of open source, security vulnerability fixes are public. An announcement room at #frontier-security:matrix.org is available. The room is invite only and is only for ecosystem users who require immediate and urgent actions when an advisory is available. Please contact [Wei](mailto:wei@parity.io) for invites.


## CHANGELOG
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