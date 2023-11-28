import { Principal } from '@dfinity/principal';
import { useEffect, useState } from 'react';
import { useCanister } from '@connect2ic/react';
import { stringToUint8Array } from '@utils/misc/buf2hex';


const queryBalance = async (principal,  actors, sub,) => {
	const subaccounts = await Promise.resolve(actors[0]
		.icrc1_balance_of(
			{owner: Principal.fromText(principal), subaccount: sub ? [stringToUint8Array(sub)] : []}
		));
	return subaccounts;
};

export const useBalance = (principal) => {
	const actor = useCanister('ledgerIndexerCanister');
	const [balance, setBalance] = useState([]);
	const [isLoading, setLoading] = useState(false);
	useEffect(() => {
		setLoading(true);
		queryBalance(principal, actor)
			.then((result) => {
				setBalance(result);
				setLoading(false);
			})
			.catch((error) => {
				console.log('error', error);
				setLoading(false);
			});
	}, []);
	return { balance: (parseInt(balance)), isLoading };
};