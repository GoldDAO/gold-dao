use std::time::Duration;

use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };

#[derive(CandidType, Deserialize, Serialize)]
pub struct Init {
    /// The canister ID of the NNS governance canister. This is the only
    /// principal that can open the swap.
    pub nns_governance_canister_id: String,
    /// The canister ID of the governance canister of the SNS that this
    /// token swap pertains to.
    pub sns_governance_canister_id: String,
    /// The ledger canister of the SNS.
    pub sns_ledger_canister_id: String,
    /// The ledger canister for the base token, typically ICP. The base
    /// token is typically ICP, but this assumption is not used anywhere,
    /// so, in principle, any token type can be used as base token.
    pub icp_ledger_canister_id: String,
    /// Analogous to `sns_governance_canister_id`, but for the "root"
    /// canister instead of the governance canister.
    pub sns_root_canister_id: String,
    /// If the swap is aborted, control of the canister(s) should be set to these
    /// principals. Must not be empty.
    pub fallback_controller_principal_ids: Vec<String>,
    /// Same as SNS ledger. Must hold the same value as SNS ledger. Whether the
    /// values match is not checked. If they don't match things will break.
    pub transaction_fee_e8s: Option<u64>,
    /// Same as SNS governance. Must hold the same value as SNS governance. Whether
    /// the values match is not checked. If they don't match things will break.
    pub neuron_minimum_stake_e8s: Option<u64>,
    /// An optional text that swap participants should confirm before they may
    /// participate in the swap. If the field is set, its value should be plain
    /// text with at least 1 and at most 1,000 characters.
    pub confirmation_text: Option<String>,
    /// An optional set of countries that should not participate in the swap.
    pub restricted_countries: Option<Countries>,
    /// The minimum number of buyers that must participate for the swap
    /// to take place. Must be greater than zero.
    pub min_participants: Option<u32>,
    /// The total number of ICP that is required for this token swap to
    /// take place. This number divided by the number of SNS tokens being
    /// offered gives the seller's reserve price for the swap, i.e., the
    /// minimum number of ICP per SNS tokens that the seller of SNS
    /// tokens is willing to accept. If this amount is not achieved, the
    /// swap will be aborted (instead of committed) when the due date/time
    /// occurs. Must be smaller than or equal to `max_icp_e8s`.
    pub min_icp_e8s: Option<u64>,
    /// The number of ICP that is "targeted" by this token swap. If this
    /// amount is achieved with sufficient participation, the swap will be
    /// triggered immediately, without waiting for the due date
    /// (`end_timestamp_seconds`). This means that an investor knows the minimum
    /// number of SNS tokens received per invested ICP. If this amount is achieved
    /// without reaching sufficient_participation, the swap will abort without
    /// waiting for the due date. Must be at least
    /// `min_participants * min_participant_icp_e8s`
    pub max_icp_e8s: Option<u64>,
    /// The total number of ICP that is required to be "directly contributed"
    /// for this token swap to take place. This number divided by the number of SNS tokens being
    /// offered gives the seller's reserve price for the swap, i.e., the
    /// minimum number of ICP per SNS tokens that the seller of SNS
    /// tokens is willing to accept. If this amount is not achieved, the
    /// swap will be aborted (instead of committed) when the due date/time
    /// occurs. Must be smaller than or equal to `max_icp_e8s`.
    pub min_direct_participation_icp_e8s: Option<u64>,
    /// The number of ICP that is "targeted" by this token swap. If this
    /// amount is achieved with sufficient participation, the swap will be
    /// triggered immediately, without waiting for the due date
    /// (`end_timestamp_seconds`). This means that an investor knows the minimum
    /// number of SNS tokens received per invested ICP. If this amount is achieved
    /// without reaching sufficient_participation, the swap will abort without
    /// waiting for the due date. Must be at least
    /// `min_participants * min_participant_icp_e8s`.
    pub max_direct_participation_icp_e8s: Option<u64>,
    /// The minimum amount of ICP that each buyer must contribute to
    /// participate. Must be greater than zero.
    pub min_participant_icp_e8s: Option<u64>,
    /// The maximum amount of ICP that each buyer can contribute. Must be
    /// greater than or equal to `min_participant_icp_e8s` and less than
    /// or equal to `max_icp_e8s`. Can effectively be disabled by
    /// setting it to `max_icp_e8s`.
    pub max_participant_icp_e8s: Option<u64>,
    /// The date/time when the swap should start.
    pub swap_start_timestamp_seconds: Option<u64>,
    /// The date/time when the swap is due, i.e., it will automatically
    /// end and commit or abort depending on whether the parameters have
    /// been fulfilled.
    pub swap_due_timestamp_seconds: Option<u64>,
    /// The number of tokens (of `init.sns_ledger_canister_id`) that are
    /// being offered. The tokens are held in escrow for the SNS
    /// governance canister.
    ///
    /// Invariant for the OPEN state:
    /// ```text
    /// state.sns_token_e8s <= token_ledger.balance_of(<swap-canister>)
    /// ```
    pub sns_token_e8s: Option<u64>,
    /// The construction parameters for the basket of neurons created for all
    /// investors in the decentralization swap. Each investor, whether via
    /// the Neurons' Fund or direct, will receive `count` Neurons with
    /// increasing dissolve delays. The total number of Tokens swapped for
    /// by the investor will be evenly distributed across the basket. This is
    /// effectively a vesting schedule to ensure there is a gradual release of
    /// SNS Tokens available to all investors instead of being liquid immediately.
    /// See `NeuronBasketConstructionParameters` for more details on how
    /// the basket is configured.
    pub neuron_basket_construction_parameters: Option<NeuronBasketConstructionParameters>,
    /// The ID of the NNS proposal submitted to launch this SNS decentralization
    /// swap.
    pub nns_proposal_id: Option<u64>,
    /// The Neurons' Fund participants of this SNS decentralization swap.
    pub neurons_fund_participants: Option<NeuronsFundParticipants>,
    /// Controls whether swap finalization should be attempted automatically in the
    /// canister heartbeat. If set to false, `finalize_swap` must be called
    /// manually. Note: it is safe to call `finalize_swap` multiple times
    /// (regardless of the value of this field).
    pub should_auto_finalize: Option<bool>,
    /// Constraints for the Neurons' Fund participation in this swap.
    pub neurons_fund_participation_constraints: Option<NeuronsFundParticipationConstraints>,
    /// Whether Neurons' Fund participation is requested.
    pub neurons_fund_participation: Option<bool>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct SnsInitPayload {
    /// Fee of a transaction.
    pub transaction_fee_e8s: Option<u64>,
    /// The name of the token issued by an SNS Ledger.
    /// This field has no default, a value must be provided by the user.
    /// Must be a string length between {} and {} characters
    ///
    /// Example: Bitcoin
    pub token_name: Option<String>,
    /// The symbol of the token issued by an SNS Ledger. This field has no
    /// default, a value must be provided by the user. Must be a string length
    /// between 3 and 10 characters
    pub token_symbol: Option<String>,
    /// Cost of making a proposal that doesnt pass.
    pub proposal_reject_cost_e8s: Option<u64>,
    /// The minimum amount of SNS Token e8s an SNS Ledger account must have to stake a neuron.
    pub neuron_minimum_stake_e8s: Option<u64>,
    /// If the swap fails, control of the dapp canister(s) will be set to these
    /// principal IDs. In most use-cases, this would be the same as the original
    /// set of controller(s). Must not be empty.
    pub fallback_controller_principal_ids: Vec<String>,
    /// The logo for the SNS project represented as a base64 encoded string.
    pub logo: Option<String>,
    /// Url to the dapp controlled by the SNS project.
    pub url: Option<String>,
    /// Name of the SNS project. This may differ from the name of the associated token.
    pub name: Option<String>,
    /// Description of the SNS project.
    pub description: Option<String>,
    /// The minimum dissolve_delay in seconds a neuron must have to be able to cast votes on proposals.
    pub neuron_minimum_dissolve_delay_to_vote_seconds: Option<u64>,
    /// The amount of rewards is proportional to token_supply * current_rate. In
    /// turn, current_rate is somewhere between these two values. In the first
    /// reward period, it is the initial growth rate, and after the growth rate
    /// transition period has elapsed, the growth rate becomes the final growth
    /// rate, and remains at that value for the rest of time. The transition
    /// between the initial and final growth rates is quadratic, and levels out at
    /// the end of the growth rate transition period.
    ///
    /// (A basis point is one in ten thousand.)
    pub initial_reward_rate_basis_points: Option<u64>,
    pub final_reward_rate_basis_points: Option<u64>,
    /// The amount of time that the growth rate changes (presumably, decreases)
    /// from the initial growth rate to the final growth rate. (See the two
    /// *_reward_rate_basis_points fields bellow.) The transition is quadratic, and
    /// levels out at the end of the growth rate transition period.
    pub reward_rate_transition_duration_seconds: Option<u64>,
    /// The maximum dissolve delay that a neuron can have. That is, the maximum
    /// that a neuron's dissolve delay can be increased to. The maximum is also enforced
    /// when saturating the dissolve delay bonus in the voting power computation.
    pub max_dissolve_delay_seconds: Option<u64>,
    /// The age of a neuron that saturates the age bonus for the voting power computation.
    pub max_neuron_age_seconds_for_age_bonus: Option<u64>,
    /// E.g. if a large dissolve delay can double the voting power of a neuron,
    /// then this field would have a value of 2.0.
    ///
    /// For no bonus, this should be set to 1.
    ///
    /// To achieve functionality equivalent to NNS, this should be set to 2.
    pub max_dissolve_delay_bonus_percentage: Option<u64>,
    /// Analogous to the previous field (see the previous comment),
    /// but this one relates to neuron age instead of dissolve delay.
    ///
    /// To achieve functionality equivalent to NNS, this should be set to 1.25.
    pub max_age_bonus_percentage: Option<u64>,
    /// The initial voting period of a newly created proposal.
    /// A proposal's voting period may then be further increased during
    /// a proposal's lifecycle due to the wait-for-quiet algorithm.
    ///
    /// The voting period must be between (inclusive) the defined floor
    /// INITIAL_VOTING_PERIOD_SECONDS_FLOOR and ceiling
    /// INITIAL_VOTING_PERIOD_SECONDS_CEILING.
    pub initial_voting_period_seconds: Option<u64>,
    /// The wait for quiet algorithm extends the voting period of a proposal when
    /// there is a flip in the majority vote during the proposal's voting period.
    /// This parameter determines the maximum time period that the voting period
    /// may be extended after a flip. If there is a flip at the very end of the
    /// original proposal deadline, the remaining time will be set to this parameter.
    /// If there is a flip before or after the original deadline, the deadline will
    /// extended by somewhat less than this parameter.
    /// The maximum total voting period extension is 2 * wait_for_quiet_deadline_increase_seconds.
    /// For more information, see the wiki page on the wait-for-quiet algorithm:
    /// <https://wiki.internetcomputer.org/wiki/Network_Nervous_System#Proposal_decision_and_wait-for-quiet>
    pub wait_for_quiet_deadline_increase_seconds: Option<u64>,
    /// An optional text that swap participants should confirm before they may
    /// participate in the swap. If the field is set, its value should be plain text
    /// with at least 1 and at most 1,000 characters.
    pub confirmation_text: Option<String>,
    /// An optional set of countries that should not participate in the swap.
    pub restricted_countries: Option<Countries>,
    /// / Canisters that will be transferred to an SNS.
    pub dapp_canisters: Option<DappCanisters>,
    /// The minimum number of buyers that must participate for the swap
    /// to take place. Must be greater than zero.
    pub min_participants: Option<u64>,
    /// The total number of ICP that is required for this token swap to
    /// take place. This number divided by the number of SNS tokens being
    /// offered gives the seller's reserve price for the swap, i.e., the
    /// minimum number of ICP per SNS tokens that the seller of SNS
    /// tokens is willing to accept. If this amount is not achieved, the
    /// swap will be aborted (instead of committed) when the due date/time
    /// occurs. Must be smaller than or equal to `max_icp_e8s`.
    pub min_icp_e8s: Option<u64>,
    /// The number of ICP that is "targeted" by this token swap. If this
    /// amount is achieved with sufficient participation, the swap will be
    /// triggered immediately, without waiting for the due date
    /// (`end_timestamp_seconds`). This means that an investor knows the minimum
    /// number of SNS tokens received per invested ICP. If this amount is achieved
    /// without reaching sufficient_participation, the swap will abort without
    /// waiting for the due date. Must be at least
    /// `min_participants * min_participant_icp_e8s`.
    pub max_icp_e8s: Option<u64>,
    /// The amount of ICP that is required to be directly contributed for this
    /// token swap to take place. This number + the minimum NF contribution divided
    /// by the number of SNS tokens being offered gives the seller's reserve price
    /// for the swap, i.e., the minimum number of ICP per SNS tokens that the
    /// seller of SNS tokens is willing to accept. If this amount is not achieved,
    /// the swap will be aborted (instead of committed) when the due date/time
    /// occurs. Must be smaller than or equal to `max_icp_e8s`.
    pub min_direct_participation_icp_e8s: Option<u64>,
    /// The amount of ICP that this token swap is "targeting" for direct
    /// contribution. If this amount is achieved with sufficient participation, the
    /// swap will be triggered immediately, without waiting for the due date
    /// (`end_timestamp_seconds`). This means that an investor knows the minimum
    /// number of SNS tokens received per invested ICP. If this amount is achieved
    /// without reaching sufficient_participation, the swap will abort without
    /// waiting for the due date. Must be at least
    /// `min_participants * min_participant_icp_e8s`.
    pub max_direct_participation_icp_e8s: Option<u64>,
    /// The minimum amount of ICP that each buyer must contribute to
    /// participate. Must be greater than zero.
    pub min_participant_icp_e8s: Option<u64>,
    /// The maximum amount of ICP that each buyer can contribute. Must be
    /// greater than or equal to `min_participant_icp_e8s` and less than
    /// or equal to `max_icp_e8s`. Can effectively be disabled by
    /// setting it to `max_icp_e8s`.
    pub max_participant_icp_e8s: Option<u64>,
    /// The date/time when the swap should start.
    pub swap_start_timestamp_seconds: Option<u64>,
    /// The date/time when the swap is due, i.e., it will automatically
    /// end and commit or abort depending on whether the parameters have
    /// been fulfilled.
    pub swap_due_timestamp_seconds: Option<u64>,
    /// The construction parameters for the basket of neurons created for all
    /// investors in the decentralization swap. Each investor, whether via
    /// the Neurons' Fund or direct, will receive `count` Neurons with
    /// increasing dissolve delays. The total number of Tokens swapped for
    /// by the investor will be evenly distributed across the basket. This is
    /// effectively a vesting schedule to ensure there is a gradual release of
    /// SNS Tokens available to all investors instead of being liquid immediately.
    /// See `NeuronBasketConstructionParameters` for more details on how
    /// the basket is configured.
    pub neuron_basket_construction_parameters: Option<NeuronBasketConstructionParameters>,
    /// The ID of the NNS proposal submitted to launch this SNS decentralization
    /// swap.
    pub nns_proposal_id: Option<u64>,
    /// Whether or not the neurons' fund is participating
    pub neurons_fund_participation: Option<bool>,
    /// The Neurons' Fund participants of this SNS decentralization swap.
    pub neurons_fund_participants: Option<NeuronsFundParticipants>,
    /// The token_logo for the SNS project represented as a base64 encoded string.
    pub token_logo: Option<String>,
    /// Constraints for the Neurons' Fund participation in this swap. These constraints passed from
    /// the NNS Governance (via SNS-W) to an SNS Swap to determine the Neurons' Fund participation
    /// amount as a function of the direct participation amount.
    pub neurons_fund_participation_constraints: Option<NeuronsFundParticipationConstraints>,
    /// The initial tokens and neurons available at genesis will be distributed according
    /// to the strategy and configuration picked via the initial_token_distribution
    /// parameter.
    pub initial_token_distribution: Option<InitialTokenDistribution>,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct Countries {
    /// ISO 3166-1 alpha-2 codes
    pub iso_codes: Vec<String>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct DappCanisters {
    pub canisters: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct NeuronBasketConstructionParameters {
    pub count: Option<u64>,
    pub dissolve_delay_interval: Option<Duration>,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct NeuronsFundParticipants {
    pub participants: Vec<CfParticipant>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CfParticipant {
    /// The principal that can vote on behalf of these Neurons' Fund neurons.
    pub hotkey_principal: String,
    /// Information about the participating neurons. Must not be empty.
    pub cf_neurons: Vec<CfNeuron>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CfNeuron {
    /// The NNS neuron ID of the participating neuron.
    pub nns_neuron_id: u64,
    /// The amount of ICP that the Neurons' Fund invests associated
    /// with this neuron.
    pub amount_icp_e8s: u64,
    /// Idempotency flag indicating whether the neuron recipes have been created for
    /// the CfNeuron. When set to true, it signifies that the action of creating neuron
    /// recipes has been performed on this structure. If the action is retried, this flag
    /// can be checked to avoid duplicate operations.
    pub has_created_neuron_recipes: Option<bool>,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct NeuronsFundParticipationConstraints {
    /// The Neurons' Fund will not participate in this swap unless the direct
    /// contributions reach this threshold (in ICP e8s).
    pub min_direct_participation_threshold_icp_e8s: Option<u64>,
    /// Maximum amount (in ICP e8s) of contributions from the Neurons' Fund to this swap.
    pub max_neurons_fund_participation_icp_e8s: Option<u64>,
    /// List of intervals in which the given linear coefficients apply for scaling the
    /// ideal Neurons' Fund participation amount (down) to the effective Neurons' Fund
    /// participation amount.
    pub coefficient_intervals: Vec<LinearScalingCoefficient>,
    /// The function used in the implementation of Matched Funding for mapping amounts of direct
    /// participation to "ideal" Neurons' Fund participation amounts. The value needs to be adjusted
    /// to a potentially smaller value due to SNS-specific participation constraints and
    /// the configuration of the Neurons' Fund at the time of the CreateServiceNervousSystem proposal
    /// execution.
    pub ideal_matched_participation_function: Option<IdealMatchedParticipationFunction>,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct LinearScalingCoefficient {
    /// (Included) lower bound on the amount of direct participation (in ICP e8s) at which
    /// these coefficients apply.
    pub from_direct_participation_icp_e8s: Option<u64>,
    /// (Excluded) upper bound on the amount of direct participation (in ICP e8s) at which
    /// these coefficients apply.
    pub to_direct_participation_icp_e8s: Option<u64>,
    /// Numerator or the slope of the linear transformation.
    pub slope_numerator: Option<u64>,
    /// Denominator or the slope of the linear transformation.
    pub slope_denominator: Option<u64>,
    /// Intercept of the linear transformation (in ICP e8s).
    pub intercept_icp_e8s: Option<u64>,
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct IdealMatchedParticipationFunction {
    /// The encoding of the "ideal" matched participation function is defined in `crate::neurons_fund`.
    /// In the future, we could change this message to represent full abstract syntactic trees
    /// comprised of elementary mathematical operators, with literals and variables as tree leaves.
    pub serialized_representation: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum InitialTokenDistribution {
    /// See `FractionalDeveloperVotingPower`
    FractionalDeveloperVotingPower(FractionalDeveloperVotingPower),
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct FractionalDeveloperVotingPower {
    /// The developer bucket.
    pub developer_distribution: Option<DeveloperDistribution>,
    /// The treasury bucket.
    pub treasury_distribution: Option<TreasuryDistribution>,
    /// The swap bucket.
    pub swap_distribution: Option<SwapDistribution>,
    /// The airdrop bucket.
    pub airdrop_distribution: Option<AirdropDistribution>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct DeveloperDistribution {
    /// List of `NeuronDistribution` that specify a Neuron controller and Neuron stake in e8s (10E-8 of a token).
    /// For each entry in the developer_neurons list, a neuron will be created with a voting multiplier applied
    /// (see `FractionalDeveloperVotingPower`) and will start in PreInitializationSwap mode.
    pub developer_neurons: Vec<NeuronDistribution>,
}
/// The funds for the SNS' Treasury account on the SNS Ledger. These funds are
/// in the SNS Ledger at genesis, but unavailable until after the initial swap
/// has successfully completed.
#[derive(CandidType, Deserialize, Serialize)]
pub struct TreasuryDistribution {
    /// The total token distribution denominated in e8s (10E-8 of a token) of the
    /// treasury bucket.
    pub total_e8s: u64,
}
/// The funds for token swaps to decentralize an SNS. These funds are in the
/// SNS Ledger at genesis.
#[derive(CandidType, Deserialize, Serialize)]
pub struct SwapDistribution {
    /// The total token distribution denominated in e8s (10E-8 of a token) of the
    /// swap bucket. All tokens used in initial_swap_amount_e8s will be
    /// deducted from total_e8s. The remaining tokens will be distributed to
    /// a subaccount of Governance for use in future token swaps.
    pub total_e8s: u64,
    /// The initial number of tokens denominated in e8s (10E-8 of a token)
    /// deposited in the swap canister's account for the initial token swap.
    pub initial_swap_amount_e8s: u64,
}
/// The distributions airdropped at SNS genesis.
#[derive(CandidType, Deserialize, Serialize)]
pub struct AirdropDistribution {
    /// List of `NeuronDistribution` that specify a Neuron controller and Neuron stake in e8s
    /// (10E-8 of a token). For each entry in the airdrop_neurons list, a neuron will be
    /// created with NO voting multiplier applied and will start in PreInitializationSwap mode.
    pub airdrop_neurons: Vec<NeuronDistribution>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct NeuronDistribution {
    /// The initial `PrincipalId` given permissions on a neuron available at genesis.
    /// The permissions granted to the controller will be set to the SNS' configured
    /// `NervousSystemParameters.neuron_claimer_permissions`. This controller
    /// will be the first available `PrincipalId` to manage a neuron.
    pub controller: Option<Principal>,
    /// The stake denominated in e8s (10E-8 of a token) that the neuron will have
    /// at genesis. The `Neuron.cached_neuron_stake_e8s` in SNS Governance and the
    /// Neuron's account in the SNS Ledger will have this value.
    pub stake_e8s: u64,
    /// The `memo` used along with the controller's `PrincipalId` to generate the subaccount
    /// of the neuron. This allows for a single `PrincipalId` to have multiple neurons as
    /// the identifier will be unique as long as the memo is unique.
    pub memo: u64,
    /// The amount of time denominated in seconds that the neuron will have its dissolve delay
    /// set to. This value cannot be changed until after the decentralization sale is complete.
    pub dissolve_delay_seconds: u64,
    /// The duration that this neuron is vesting.
    ///
    /// A neuron that is vesting is non-dissolving and cannot start dissolving until the vesting duration has elapsed.
    /// Vesting can be used to lock a neuron more than the max allowed dissolve delay. This allows devs and members of
    /// a particular SNS instance to prove their long-term commitment to the community. For example, the max dissolve delay
    /// for a particular SNS instance might be 1 year, but the devs of the project may set their vesting duration to 3
    /// years and dissolve delay to 1 year in order to prove that they are making a minimum 4 year commitment to the
    /// project.
    pub vesting_period_seconds: Option<u64>,
}
