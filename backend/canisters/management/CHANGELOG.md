# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.1] - 2024-09-04

#### Changed
- **Dependencies**: Updated Rust CDK dependencies to the latest versions to improve compatibility and performance.

### [1.0.0] - 2024-07-16

#### Description
This marks the initial release of the management canister. Currently it provides state to indicate if the GLD Dashboard should be in maintenance mode or not. 

#### Added
- **Maintenance mode** : This is a simple piece of state that acts like a boolean on/off switch. When maintenance mode is activated the GLD Dashboard will show a maintenance page. This maintenance page is needed when both the frontend and backend canisters have made changes that rely on each other being up to date. With this feature in place we can now toggle the frontend into the maintenance mode and then update the backend canisters whilst ensuring no bad requests are made to it from an out of date frontend.