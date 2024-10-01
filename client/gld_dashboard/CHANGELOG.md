# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.7] - 2024-10-1 

### Fixed

- **Bug fix : voting power** - voting power would not show in reward neurons list if the delay was exactly the same as the nns configured minimum delay amount.

### Improvments

- **Neuron Linking** - You no longer need to link a neuron by calling a canister endpoint. As long as a dashboard principal has been added to the neuron as a hotkey, it will automatically be shown in the user's reward neuron list.
- **Deployment** - small change to deployment to only built into a dist folder if the node env is production. this allows us to develop locally.
- **Charts** - Staked, Liquid, Burned and Holders charts are now displaying data. Some bugs relating to chosing different date filters for the charts have also been fixed

### [1.0.6] - 2024-08-23

#### Fixed

- **Deployment** : Fix deployment environnement issue

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
