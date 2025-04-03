# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

#### Add

- Add a method 'manual_token_transfer' to enable to let the DAO transfer funds from the canister in case needed.

#### Fix

- Correct the owner of a newly created SNS neuron to be the canister, not the caller

### [1.0.1] - 2025-03-25

#### Description

Changes get_apy to get_apy_overall which represents an APY in that instant. Also adds a new endpoint get_apy_timeseries which returns a set of weekly timestamps and corresponding APY snapshots

### [1.0.0] - 2025-03-10

#### Description

This marks the first version deployment of the GLDT Stake backend canister. Giving the GOLD DAO project the ability to stake GLDT to receive rewards.
