use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, CandidType, PartialEq)]
pub enum VoteType {
    SelfVote,     // vote cast by the neuron itself
    FolloweeVote, // vote cast by a followee
}
