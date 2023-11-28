import { Principal } from '@dfinity/principal';
import { useEffect, useState } from 'react';
import { useCanister } from '@connect2ic/react';
import {  stringToUint8Array } from '@utils/misc/buf2hex';

const queryHistory = async (principal, actors, currentPage, currentSub, i) => {
	const history = await Promise.resolve(actors[0]
		.get_account_transactions({
			max_results: 10,
			start: currentPage > 0 ? [i[currentPage]] : i[0],  
			account: {
				owner: Principal.fromText(principal), 
				subaccount: currentSub ? [stringToUint8Array(currentSub)] : []
			}
		}));
	return {history} ;
};

 
export const useHistory = (principal, currentPage,  currentSub, i ) => {
	const actor = useCanister('ledgerIndexerCanister');
	const [history, setHistory] = useState();
	const [isLoading, setLoading] = useState(false);

	useEffect(() => {
		setHistory();
		setLoading(true);
		const fetch = async () => {
			await queryHistory(principal, actor, currentPage,  currentSub, i)
				.then((result) => {
					setHistory(result);
					setLoading(false);
				})
				.catch((error) => {
					console.log('error', error);
					setLoading(false);
				});
		};
		fetch();
	}, [currentPage,  currentSub]);
	return {isLoading, history};
};
