# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.5] - 2024-08-23

#### Fixed

- **Analytics** : Fixed analytics

### [1.0.4] - 2024-08-23

#### Fixed

- **Claiming only OGY** : Fixed disabled claim when only OGY available

### [1.0.3] - 2024-08-21

#### Fixed

- **Lock periods and voting powers** : Fixed display of some voting power calculations and dissolve delays for neurons

#### Added

- **GLDGov OGY neuron** : added display for OGY neurons on the dashboard
- **Use async loading for neurons** : connected beurons are loading asynchronously now
- **OGY balance and rewards** : OGY token balance and rewards claiming enabled

### [1.0.2] - 2024-07-25

#### Fixed

- **Transfer modal**: fix the issue that when clicking on "Max" in the transfer modal, the e8s representation would be pasted. Now it takes the correct decimal value.

### [1.0.1] - 2024-07-23

#### Description

This includes fixes and improvements

#### Added

- **Rewards claiming** : adjust the reward claiming to work with the new version of the sns_rewards canisters

### [1.0.0] - 2024-05-01

#### Description

This marks the initial release of Gold DAO dashboard. It allows users to view essential information about GLDGov and let's users claim rewards from staking.
