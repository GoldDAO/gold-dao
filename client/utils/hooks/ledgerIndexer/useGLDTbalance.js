import { useCanister} from '@connect2ic/react';
import { useState, useEffect } from 'react';
import { Principal } from '@dfinity/principal';

const queryGLDTbalance = async (actor, principal) => {
	const req = await actor[0].icrc1_balance_of({ owner: principal, subaccount: [] });
	return req;
};

const useGLDTbalance = (principal, shouldUpdate) => {
	const [balance, setBalance] = useState([]);
	const [isLoading, setIsloading] = useState(false);
	const gldtLedgerActor = useCanister('gldtLedgerCanister');
	useEffect(() => {
		if (principal) {
			const fetchBalance = async () => {
				setIsloading(true);
				const fetchedBalance = await queryGLDTbalance(
					gldtLedgerActor,
					Principal.fromText(principal),
				);
				setBalance((Number(fetchedBalance) / 100000000).toFixed(2));
				setIsloading(false);

			};
			fetchBalance();
		}
		
	}, [principal, shouldUpdate]);
	return {balance, isLoading};
};

export default useGLDTbalance;
