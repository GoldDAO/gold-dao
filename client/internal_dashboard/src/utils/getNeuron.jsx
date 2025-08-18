import {GLDT_STAKE_SOURCE_NEURON_ID, GLDT_STAKE_SOURCE_NEURON_ID_IC} from '../constants';

export const getNeuron = (env) => {
  const neuron = {
    staging: {
      GLDT_STAKE_SOURCE_NEURON_ID: GLDT_STAKE_SOURCE_NEURON_ID,
    },
    production: {
      GLDT_STAKE_SOURCE_NEURON_ID: GLDT_STAKE_SOURCE_NEURON_ID_IC,
    },
  };
  return neuron[env];
};
