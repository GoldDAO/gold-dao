pub mod sync_governance;
pub mod sync_supply_data;
pub mod update_balance_list;
pub mod sync_governance_history;
pub mod sync_proposals_stats;

pub(crate) fn start() {
    // Computes the staked value for the last 2k days
    sync_governance_history::start_job();
    sync_proposals_stats::start_job();
    // Computes the governance stats, total staked, rewards
    // Updates the balance list (ledger + governance) for each acc
    // Also calculates circulating supply
    sync_governance::start_job();
    update_balance_list::start_job();
}
