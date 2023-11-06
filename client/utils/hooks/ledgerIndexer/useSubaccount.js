import { Principal } from '@dfinity/principal';
import { useEffect, useState } from 'react';
import { useCanister } from '@connect2ic/react';

const querySubaccounts = async (principal, actors) => {
	const subaccounts = await Promise.resolve(actors[0]
		.list_subaccounts({
			owner: Principal.fromText(principal),
			start: [],
		}));
	return subaccounts;
};

export const useSubaccounts = (principal) => {
	const actor = useCanister('ledgerIndexerCanister');
	const [subaccounts, setSubaccounts] = useState([]);
	const [isLoading, setLoading] = useState(false);

	useEffect(() => {
		setLoading(true);
		querySubaccounts(principal, actor)
			.then((result) => {
				setSubaccounts(result);
				setLoading(false);
			})
			.catch((error) => {
				console.log('error', error);
				setLoading(false);
			});
	}, []);
	return { subaccounts, isLoading };
};
