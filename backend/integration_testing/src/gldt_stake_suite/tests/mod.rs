pub mod add_stake;
pub mod claim_rewards;
pub mod dissolve_instantly;
pub mod fuzz_testing;
pub mod health_check;
pub mod metrics;
pub mod process_rewards;
// pub mod proposal_voting;
pub mod start_dissolving;
pub mod test_icrc3;
pub mod user_flows;
pub mod withdraw;

#[cfg(test)]
mod tests {
    use super::*;
}
