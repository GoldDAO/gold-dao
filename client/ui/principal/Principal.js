import React, { useEffect , useState } from 'react';
import { HStack, Text, Tooltip , Box} from '@chakra-ui/react';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';
import { useConnect } from '@connect2ic/react';

const PrincipalFormat = ({ nobtn, full, principal}) => {
	const myPrincipal = useConnect().principal;
	const [currentPrincipal, setCurrentPrincipal] = useState();

	useEffect(() => {
		if(!principal){
			setCurrentPrincipal(myPrincipal);
		} else {
			setCurrentPrincipal(principal);
		}
	},[myPrincipal]);

	const charsCount = 4;
	const firstChars = currentPrincipal?.slice(0, charsCount) || '';
	const lastChars = currentPrincipal?.substring(currentPrincipal.length - charsCount) || '';
	return (
		currentPrincipal && (
			<HStack justifySelf={'flex-end'}>
				<Tooltip label={currentPrincipal.toString()}>
					<Box>
						{!full &&
					<Text fontSize={'inherit'}>
						{firstChars}...{lastChars}
					</Text>}
						{full &&
					<Text fontSize={'inherit'}>
						{currentPrincipal}
					</Text>}
					</Box>
				</Tooltip>
				{!nobtn && <CopyPrincipal text={currentPrincipal} />}
			</HStack>
		)
	);
};

export default PrincipalFormat;
