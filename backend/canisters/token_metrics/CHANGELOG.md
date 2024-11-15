# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.5] - 2024-11-15

#### Changed

- **State**: Updated most of the data variable to use stable instead of heap memory, using `ic_stable_structures`.

### [1.0.4] - 2024-09-25

#### New

- **State**: Include versioning and commit info in state.

### [1.0.3] - 2024-09-04

- Updated Rust CDK dependencies to the latest versions to improve compatibility and performance.

### [1.0.2] - 2024-08-13

- Migrated the current version from the ogy repo which includes the comprehensive balance lists, supply data and governance related data

### [1.0.1] - 2024-06-26

#### Added

- Add http requests to expose total supply and circulating supply.

### [1.0.0] - 2024-04-02

#### Added

- first release /gold-nft-metrics endpoint for data about "total value locked", "gold_price" and "total_gold_grams"
