import { Principal } from '@dfinity/principal';


export const transfer = async (amount, to, actor) => {
	try {
		const res = await actor.icrc1_transfer({
			to: {
				owner: Principal.fromText(to),
				subaccount: [],
			},
			amount: BigInt(amount * 100000000),
			fee: [],
			from_subaccount: [],
			created_at_time: [],
			memo: [],
		});
		return res;
	} catch (e) {
		console.log('e', e);
	}
};