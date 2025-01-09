use crate::state::{mutate_state, read_state};
use candid::Principal;
use serde::{Deserialize, Serialize};
use sns_governance_canister::types::{ListNeurons, Neuron};
use tracing::error;
use tracing::info;
use utils::env::Environment;

#[derive(Serialize, Deserialize, Clone)]
pub struct NeuronSystem {
    pub neurons: Vec<Neuron>,
}

impl Default for NeuronSystem {
    fn default() -> Self {
        Self {
            neurons: Default::default(),
        }
    }
}

impl NeuronSystem {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        NeuronSystem { neurons }
    }

    pub fn set_neurons(&mut self, neurons: Vec<Neuron>) {
        self.neurons = neurons;
    }

    pub fn get_neurons(&self) -> Vec<Neuron> {
        self.neurons.clone()
    }
}

// AsRef for immutable access to the slice of neurons
impl AsRef<[Neuron]> for NeuronSystem {
    fn as_ref(&self) -> &[Neuron] {
        &self.neurons
    }
}

// AsMut for mutable access to the slice of neurons
impl AsMut<[Neuron]> for NeuronSystem {
    fn as_mut(&mut self) -> &mut [Neuron] {
        &mut self.neurons
    }
}

// Fetch all neurons from SNS governance canister
async fn fetch_neurons(
    sns_governance_canister_id: Principal,
    canister_id: Principal,
) -> Result<Vec<Neuron>, String> {
    let limit = 100;

    let mut args = ListNeurons {
        limit,
        start_page_at: None,
        of_principal: Some(canister_id),
    };

    let mut continue_scanning = true;

    let mut neurons = Vec::new();
    while continue_scanning {
        continue_scanning = false;

        match sns_governance_canister_c2c_client::list_neurons(sns_governance_canister_id, &args)
            .await
        {
            Ok(response) => {
                let number_of_received_neurons = response.neurons.len();
                if (number_of_received_neurons as u32) == limit {
                    args.start_page_at = response.neurons.last().map_or_else(
                        || {
                            error!(
                                "SYNC NEURONS :: Missing last neuron to continue iterating.
                                This should not be possible as the limits are checked. Stopping loop here."
                            );
                            None
                        },
                        |n| {
                            continue_scanning = true;
                            n.id.clone()
                        }
                    );
                }
                neurons.extend(response.neurons);
            }
            Err(e) => {
                error!("SYNC NEURONS :: Failed to obtain all neurons data {:?}", e);
                return Err(format!("Failed to obtain all neurons data {:?}", e));
            }
        }
    }
    Ok(neurons)
}

pub async fn sync_neurons() -> Result<(), String> {
    info!("SYNC NEURONS :: start");
    let (this_canister_id, sns_governance_canister_id) =
        read_state(|s| (s.env.canister_id(), s.data.gld_sns_governance_canister_id));

    // Error is handled in fetch_neurons
    let neurons = fetch_neurons(sns_governance_canister_id, this_canister_id).await?;

    mutate_state(|s| s.data.neuron_system.set_neurons(neurons));
    info!("SYNC NEURONS :: neuron data synced successfully");
    Ok(())
}
