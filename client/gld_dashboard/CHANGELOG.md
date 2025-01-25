# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.14] - 2025-01-25

#### Fix

- GOLDAO price

### [1.0.13] - 2025-01-24

#### Fix

- Typogaphy error on the claim reward preventing it from working properly
- GOLDAO price

### [1.0.12] - 2025-01-23

#### Changed

- Rename GLDGov ticker to GOLDAO

### [1.0.11] - 2024-11-18

#### Fixes

- **Fix treasury chart**: On mobile devices, there was a bug when showing the treasury amount, this has now been fixed.

### [1.0.10] - 2024-11-14

#### Improvements

- **Update Chart Descriptions**: descriptions on the charts are now more in line with what the charts display.
- **Update Liquid Chart**: Display the liquid chart as circulating supply minus treasury as per industry standard.

### [1.0.9] - 2024-10-29

#### Fixed

- Update alternative origins to include gldt canisters

### [1.0.8] - 2024-10-21

#### Fixed

- Update logos

### [1.0.7] - 2024-10-10

#### Fixed

- **Bug fix : voting power** - voting power would not show in reward neurons list if the delay was exactly the same as the nns configured minimum delay amount.

#### Improvments

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
