import React, { useEffect } from 'react';
import { HStack, Text, Tooltip , Box} from '@chakra-ui/react';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';
import { useConnect } from '@connect2ic/react';

const PrincipalFormat = ({ nobtn, full, principal}) => {
	const myPrincipal = useConnect().principal;
	useEffect(() => {
		if(!principal){
			principal = myPrincipal;
		}
	},[]);

	const charsCount = 4;
	const firstChars = principal?.slice(0, charsCount) || '';
	const lastChars = principal?.substring(principal.length - charsCount) || '';
	console.log('principal', principal);
	return (
		principal && (
			<HStack justifySelf={'flex-end'}>
				<Tooltip label={principal.toString()}>
					<Box>
						{!full &&
					<Text fontSize={'inherit'}>
						{firstChars}...{lastChars}
					</Text>}
						{full &&
					<Text fontSize={'inherit'}>
						{principal}
					</Text>}
					</Box>

				</Tooltip>
				{!nobtn && <CopyPrincipal text={principal} />}
			</HStack>
		)
	);
};

export default PrincipalFormat;
