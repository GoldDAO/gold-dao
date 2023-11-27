import { useCanister } from '@connect2ic/react';
import { useEffect,useState } from 'react';

const queryGLDTbalance = async (actor) => {
	const req = await actor[0].icrc1_total_supply();
	return req;
};

export const useTotalSupply = () => {
	const [totalSupply, setTotalSupply] = useState([]);
	const gldtLedgerActor = useCanister('gldtLedgerCanister',{ mode: 'anonymous' });
	useEffect(() => {
		const fetchSupply = async () => {
			const fetchedtotalSupply = await queryGLDTbalance(gldtLedgerActor);
			setTotalSupply(
				{
					gldt: (Number(fetchedtotalSupply) / 100000000).toFixed(2),
					g: (Number(fetchedtotalSupply) / 100000000).toFixed(2) * 100
				}
			);
		};
		fetchSupply();
	}, []);
	return totalSupply;
};

export default useTotalSupply;