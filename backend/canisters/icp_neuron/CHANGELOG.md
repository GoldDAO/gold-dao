# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.1.5] - 2026-07-14

- Set neuron's visibility to public

### [1.1.4] - 2025-10-22

- Fix in cycles account conversion

### [1.1.3] - 2025-10-17

#### Updated

- Update cycles management account
  
### [1.1.2] - 2025-07-18

#### Added

- **Support of ICRC10 standard**: ICRC-10 is a standard aimed at simplifying the discovery of supported standards by canisters on the Internet Computer. By providing a unified method, icrc10_supported_standards, canisters can easily expose the standards they implement, enhancing interoperability and easing integration efforts across the ecosystem.

### [1.1.1] - 2025-02-07

- **Refresh neurons**: With DFINITY introducing the 6 month refresh neuron restriction, the followees of the NNS neurons of the Gold DAO need to also be refreshed every six months. The API had to be updated to enable proposals to call the RefreshVotingPower command on the canister.

### [1.1.0] - 2024-11-12

- **Cycle Management**: This feature transfers from the DAO ICP neurons to a cycle management platform which in turn
distributes converts ICP to cycles to be used for DAO owned canisters. If the cycle wallet on the platform is below 1000 ICP then spawned maturity is used to top up. This ensures the DAO's canisters can be funded with a continous and reliable supply of cycles.

### [1.0.3] - 2024-09-25

#### New

- **State**: Include versioning and commit info in state.

### [1.0.2] - 2024-09-04

#### Changed

- **Dependencies**: Updated Rust CDK dependencies to the latest versions to improve compatibility and performance.
  
### [1.0.1] - 2024-03-26

#### Added

- enabled automatic maturity disbursement when 1000 ICP are accumulated in maturity
  
### [1.0.0] - 2024-03-15

#### Added

- first release with all management methods of the ICP neuron
