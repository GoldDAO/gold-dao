import { Principal } from '@dfinity/principal';
import { useEffect, useState } from 'react';
import { useCanister } from '@connect2ic/react';

const queryHistory = async (principal, actors, currentPage, currentSub, i) => {
	const history = await Promise.resolve(actors[0]
		.get_account_transactions({
			max_results: 10,
			start: currentPage > 0 ? [i[currentPage]] : i[0],  
			account: {
				owner: Principal.fromText(principal), 
				subaccount: currentSub ? [Principal.fromText(currentSub)] : []
			}
		}));
	return {history} ;
};

 
export const useHistory = (principal, currentPage,  currentSub, i ) => {
	const actor = useCanister('ledgerIndexerCanister');
	const [history, setHistory] = useState([]);
	const [isLoading, setLoading] = useState(false);
	useEffect(() => {
		setLoading(true);
		queryHistory(principal, actor, currentPage,  currentSub, i)
			.then((result) => {
				console.log('result', result);
				setHistory(result.history);
				setLoading(false);
			})
			.catch((error) => {
				console.log('error', error);
				setLoading(false);
			});
	}, [currentPage]);
	return { history, isLoading };
};
