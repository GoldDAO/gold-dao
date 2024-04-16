use crate::generate_pocket_query_call;

generate_pocket_query_call!(get_all_neurons);

// Updates
// generate_update_call!(icrc1_transfer);

pub mod get_all_neurons {
    use super::*;

    pub type Args = ();
    pub type Response = u64;
}
