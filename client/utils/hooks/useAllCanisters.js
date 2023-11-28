import { gldNftCanisters } from '@utils/agents';
import { useCanister } from '@connect2ic/react';

export const useAllCanisters = () => {
	const weights = Object.keys(gldNftCanisters);

	const actor1 = useCanister(weights[0])[0];
	const actor10 = useCanister(weights[1])[0];
	const actor100 = useCanister(weights[2])[0];
	const actor1000 = useCanister(weights[3])[0];

	const stagingActors = [actor1, actor10];

	const prodActors = [actor1, actor10, actor100, actor1000];

	return process.env.DFX_NETWORK === 'ic' ? prodActors : stagingActors;
};
