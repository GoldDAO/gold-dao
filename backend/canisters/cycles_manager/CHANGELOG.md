# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.0] - 2024-06-12

#### Description
This marks the initial release of Cycles Manager (cycles_manager canister). The Cycles Manager canister is designed to monitor and manage the cycles balance of canisters. It ensures that canisters do not run out of cycles, which are necessary for their operation. The canister performs hourly checks to track the cycles balance and initiates top-ups when needed.

#### Added
- **Daily SNS root Synchronization** : Tracks each canister's cycles balance hourly and logs the data for monitoring and analysis.
- **Automatic Top-ups** : Automatically initiates cycles top-ups for canisters that fall below a configured threshold to prevent service interruptions.