/*!
# SNS neuron maturity process

This job is responsible for processing the maturity of neurons. It is run every
epoch and processes the maturity of all neurons in this epoch. This maturity
is stored in the canister and is used to determine the rewards that a neuron
is eligible for.
*/

pub fn process_neurons() {
    // 1. fetch maturity of all neurons
    //    a. call `list_neurons` from SNS governance canister
    //    b. update the internal structure of maturity storage
}
