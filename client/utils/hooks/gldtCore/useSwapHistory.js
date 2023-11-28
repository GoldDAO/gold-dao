import { useCanister, useConnect } from '@connect2ic/react';
import { useEffect, useState } from 'react';
import { Principal } from '@dfinity/principal';

const queryHistory = async (actor, principal, page, limit) => {
	const history = await Promise.resolve(
		actor[0].get_historical_swaps_by_user({
			page: [page],
			limit:  limit ? [limit] : [],
			account: [{ owner: Principal.fromText(principal), subaccount: [] }],
		}),
	);
	return history;
};

const useSwapHistory = (page, limit, id) => {
	const [history, setHistory] = useState();
	const [isLoading, setIsloading] = useState(true);
	const principal = id ? id : useConnect().principal;
	const gldtCoreActor = useCanister('gldtCoreCanister');
	useEffect(() => {
		setIsloading(true);
		const fetchHistory = async () => {
			await queryHistory(gldtCoreActor, principal, page, limit)
				.then((result) => {
					setHistory(result);
					setIsloading(false);
				})
				.catch((error) => {
					console.log('error', error);
					setIsloading(false);
				});
		};
		fetchHistory();
	}, [page, limit, principal]);
	return { history, isLoading };
};

export default useSwapHistory;

export const useMaxEntry = (id) => {
	const [max, setMax] = useState();
	const [isLoading, setIsloading] = useState(true);
	const principal = id ? id : useConnect().principal;
	const gldtCoreActor = useCanister('gldtCoreCanister');
	useEffect(() => {
		setIsloading(true);
		const fetchHistory = async () => {
			await queryHistory(gldtCoreActor, principal, 1, 10)
				.then((result) => {
					setMax(result.Ok.total);
					setIsloading(false);
				})
				.catch((error) => {
					setIsloading(false);
					console.log('error', error);
				});
		};
		fetchHistory();
	}, []);

	return { max, isLoading };
};
