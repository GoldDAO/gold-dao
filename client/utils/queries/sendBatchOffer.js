import { gldNftCanisters } from '@utils/agents';
import { Principal } from '@dfinity/principal';

export const sendBatchOffer = async (actors, cart) => {
	const weights = Object.keys(gldNftCanisters);
	const priceRatio = 100 * 10 ** 8;
	const gldNftCart = {};
	const YUMI_KYC_CANISTER_ID = process.env.YUMI_KYC_CANISTER_ID;

	const payload = (e) => {
		const salePrice = e.weight * priceRatio;
		return {
			token_id: e.name,
			sales_config: {
				escrow_receipt: [],
				broker_id: [],
				pricing: {
					ask: [
						[
							{
								kyc: Principal.fromText(YUMI_KYC_CANISTER_ID),
							},
							{
								buy_now: salePrice,
							},
							{ notify: [Principal.fromText(process.env.GLDT_CANISTER_ID)] },
							{
								token: {
									ic: {
										standard: { ICRC1: null },
										canister: Principal.fromText(
											process.env.GLDT_LEDGER_CANISTER_ID,
										),
										decimals: 8,
										fee: [10000],
										symbol: 'GLDT',
										id: [],
									},
								},
							},
						],
					],
				},
			},
		};
	};

	cart.map((e) => {
		if (gldNftCart[e.weight]) {
			gldNftCart[e.weight].push(payload(e));
		} else {
			gldNftCart[e.weight] = [payload(e)];
		}
	});

	const res = await Promise.all(
		weights.map((w, i) => {
			const w_int = +w.slice(0, -1);
			if (gldNftCart[w_int]) {
				return actors[i].market_transfer_batch_nft_origyn(gldNftCart[w_int]);
			} else return undefined;
		}),
	);

	return res;
};
