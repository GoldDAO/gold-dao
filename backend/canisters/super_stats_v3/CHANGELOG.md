# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.0.4] - 2024-08-07

- Added checksum validation for account queries.
- Fixed an issue where the history for account/principals balances was returning 0 when fewer days were used as an input. 

## [1.0.3] - 2024-06-08

- Added the dashboard account of foundation to the init args

## [1.0.2] - 2024-06-05

- The `super_stats_v3` canister now exports the candid and can be auto-generated, similar other backend canisters.

## [1.0.1] - 2024-06-03

- Now computes the `max_balance` for each account/principal based on transaction blocks. This will be returned by `get_principal_overview` and `get_account_overview`.
