# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]
  
### [1.0.0] - 2024-07-18

#### Description
This marks the initial release of SNS Neuron Controller Canister (sns_neuron_controller canister). It implements the NeuronManager trait, providing functionality to interact with the SNS governance canisters. Key features include daily processing of neurons, claiming rewards, and distributing these rewards based on specified thresholds.

#### Added

- first release with all management methods of any SNS neuron
- Introduced NeuronManager trait for managing SNS neurons
- Implemented NeuronManager functionality for OGYNeuron:
    - fetch_and_sync_neurons
    - get_available_rewards
    - claim_rewards
    - distribute_rewards
