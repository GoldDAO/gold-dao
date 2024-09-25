# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

#### New

- **State**: Include versioning and commit info in state.

### [1.0.4] - 2024-09-04

#### Changed

- **Dependencies**: Updated Rust CDK dependencies to the latest versions to improve compatibility and performance.

### [1.0.3] - 2024-08-20

#### Description

#### Fixed

- **Error handling** : Enhancing error handling while claiming rewards.
- **Job Scheduling** : Deleted unnecessary run function recursive call.

#### Added

- get_available_rewards query

### [1.0.2] - 2024-08-06

#### Description

#### Fixed

- **Neuron Metrics** : Fixed an issue where the incorrect deposit account was displayed for neuron metrics due to incorrect account owner specification. The fix involved extending the NeuronManager trait with a get_neuron_metrics function that automatically handles adding the respective SNS governance canister ID.

### [1.0.1] - 2024-08-06

#### Changed

- correct temporary authorized_principal for initial setup
  
### [1.0.0] - 2024-07-18

#### Description

This marks the initial release of SNS Neuron Controller Canister (sns_neuron_controller canister). It implements the NeuronManager trait, providing functionality to interact with the SNS governance canisters. Key features include daily processing of neurons, claiming rewards, and distributing these rewards based on specified thresholds.

#### Added

- first release with all management methods of any SNS neuron
- Introduced NeuronManager and NeuronRewardsManager traits for managing SNS neurons with the following methods:
  - fetch_and_sync_neurons
  - get_available_rewards
  - claim_rewards
  - distribute_rewards
- Implemented NeuronManager functionality for OGYNeuron
- Implemented NeuronRewardsManager functionality for OGYNeuron
