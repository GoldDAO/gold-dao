import { Principal } from '@dfinity/principal';
import { useEffect, useState } from 'react';
import { useCanister } from '@connect2ic/react';

const queryHistory = async (principal, actors, currentPage, action, index, last, currentSub) => {
	console.log('action', action);
	console.log('index', index);
	const start = action === -1 ? [index.first] : action === +1 ? [index.last] : [];
	console.log('start', start);
	const history = await Promise.resolve(actors[0]
		.get_account_transactions({
			max_results: 10,
			start: start[0] ? start : [],  
			account: {
				owner: Principal.fromText(principal), 
				subaccount: currentSub ? [Principal.fromText(currentSub)] : []
			}
		}));
	// console.log('last', last);
	// const history = await Promise.resolve(actors[0]
	// 	.get_account_transactions({
	// 		max_results: 10,
	// 		start:  last[currentPage] ? [last[currentPage] ] : [] ,
	// 		account: {
	// 			owner: Principal.fromText(principal), 
	// 			subaccount:[]
	// 		}
	// 	}));

	return {history} ;
};

 
export const useHistory = (principal, currentPage, action, index, last, currentSub ) => {
	const actor = useCanister('ledgerIndexerCanister');
	const [history, setHistory] = useState([]);
	const [isLoading, setLoading] = useState(false);
	useEffect(() => {
		setLoading(true);
		queryHistory(principal, actor, currentPage, action, index, last, currentSub)
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
