import useGLDTbalance from '@utils/hooks/useGLDTbalance';
import { useConnect } from '@connect2ic/react';
import React, {useReducer} from 'react';
import TokenSign from '../gldt/TokenSign';
import { HStack, Text, Button } from '@chakra-ui/react';
import { RepeatIcon } from '@chakra-ui/icons';
const Balance = () => {
	const { principal } = useConnect();
	const balance = useGLDTbalance(principal);
	const [ignored, forceUpdate] = useReducer(x => x + 1, 0);



	return (
		<HStack height={0}>
			<RefreshButton forceUpdate={forceUpdate} /><Text fontSize={'16px'}>{Number(balance).toLocaleString('en-US')}</Text> <TokenSign /> 
		</HStack>
	);
};

export default Balance;

const RefreshButton = ({forceUpdate}) => {
	return(
		<Button
			onClick={() => forceUpdate()}
			borderRadius={'200px'}
			_hover={{
				bg: 'bg',
				
			}}
			h={'40px'}
			w={'40px'}
			bg="transparent"
			color="black"
			aria-label="Refresh Balance"
		>
			<RepeatIcon />
		</Button>
	);
	
};