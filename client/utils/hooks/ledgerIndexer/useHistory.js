import { Principal } from '@dfinity/principal';
import { useEffect, useState } from 'react';
import { useCanister } from '@connect2ic/react';

const queryHistory = async (principal, actors, currentPage, action, index, last, currentSub, i) => {
	console.log('action', action);
	console.log('index', index);
	console.log('i', i);
	const start = action === -1 ? [index.first] : action === +1 ? [index.last] : [];
	console.log('start', start);
	const history = await Promise.resolve(actors[0]
		.get_account_transactions({
			max_results: 10,
			start: currentPage > 0 ? [i[currentPage]] : i[0],  
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

 
export const useHistory = (principal, currentPage, action, index, last, currentSub, i ) => {
	const actor = useCanister('ledgerIndexerCanister');
	const [history, setHistory] = useState([]);
	const [isLoading, setLoading] = useState(false);
	useEffect(() => {
		setLoading(true);
		queryHistory(principal, actor, currentPage, action, index, last, currentSub, i)
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
