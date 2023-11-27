import { useEffect, useState } from 'react';
import { useCanister } from '@connect2ic/react';


const queryBlock = async (actors, start, length, id) => {
	if(id){
		const blocks = await Promise.resolve(actors[0]
			.get_blocks(
				{
					start: parseInt(id), 
					length: 1
				}
			));
		return blocks;

	} else {
		const max = await Promise.resolve(actors[0]
			.get_blocks(
				{
					start: parseInt(1), 
					length: parseInt(1)
				}
			));
		const blocks = await Promise.resolve(actors[0]
			.get_blocks(
				{
					start: parseInt(max.chain_length) - parseInt(length) + start, 
					length: parseInt(length)
				}
			));
		return blocks;

	}

};

export const useBlock = (start, length, id) => {
	const actor = useCanister('ledgerIndexerCanister');
	const [blocks, setBlock] = useState([]);
	const [isLoading, setLoading] = useState(false);
	useEffect(() => {
		setLoading(true);
		queryBlock(actor, start, length, id)
			.then((result) => {
				result.blocks.reverse();
				setBlock(result);
				setLoading(false);
			})
			.catch((error) => {
				setLoading(false);
				console.log('error', error);
			});
	}, [start]);
	return { blocks, isLoading };
};