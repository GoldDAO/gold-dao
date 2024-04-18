# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.0] - 2024-04-18

#### Description
This release marks the first release of the SNS Rewards ( sns_rewards canister ). The sns_rewards calculates each GLDGov neuron's contribution as a percentage of the total for a given period. With the percentage we then and subsequently transfer any of the DAO's treasury neuron rewards based on said percentage.

#### Added
- **Daily GLDGov Neuron Synchronization** : Keeps track of each GLDGov neuron's maturity gains on a daily basis.
- **Weekly Reward Distribution** : Calculate and distribute rewards weekly based on the proportional accumulated maturity of each neuron for that week.
- **Daily Reserve Pool Transfer** : Transfers GLDGov tokens to a dedicated reward pool within the canister daily, ensuring a consistent payout amount each week for the weekly reward distribution of GLDGov tokens.
- **Neuron Ownership & Reward Claims** : Easily claim ownership of a neuron via hotkeys and subsequently claim any distributed rewards.