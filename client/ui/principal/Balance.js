import useGLDTbalance from '@utils/hooks/ledgerIndexer/useGLDTbalance';
import { useConnect } from '@connect2ic/react';
import React, { useState} from 'react';
import TokenSign from '../gldt/TokenSign';
import { HStack, Text, Button } from '@chakra-ui/react';
import { RepeatIcon, SpinnerIcon } from '@chakra-ui/icons';
const Balance = () => {
	const { principal } = useConnect();
	const [shouldUpdate, forceUpdate] = useState(0);
	const {balance, isLoading} = useGLDTbalance(principal, shouldUpdate);

	const handleRefresh = () => {
		forceUpdate(prevKey => prevKey + 1);
	};

	return (
		<HStack>
			<Text fontSize={'16px'}>{Number(balance).toLocaleString('en-US')}</Text> 
			<TokenSign /> 
			<RefreshButton 
				handleRefresh={handleRefresh} 
				isLoading={isLoading}/>
		</HStack>
	);
};

export default Balance;

const RefreshButton = ({handleRefresh,isLoading}) => {
	return(
		<Button
			onClick={handleRefresh}
			borderRadius={'200px'}
			_hover={{
				bg: 'bg',
				
			}}
			h={'40px'}
			w={'40px'}
			bg="transparent"
			color="black"
			aria-label="Refresh Balance"
		>{isLoading ? <SpinnerIcon /> : <RepeatIcon />}
		</Button>
	);
	
};