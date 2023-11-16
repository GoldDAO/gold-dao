import React, { useEffect } from 'react';
import { HStack, Text, Tooltip } from '@chakra-ui/react';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';
import { useConnect } from '@connect2ic/react';

const PrincipalFormat = ({ nobtn, full}) => {
	const { principal } = useConnect();
	const charsCount = 3;
	const firstChars = principal?.slice(0, charsCount);
	const lastChars = principal?.substring(principal.length - charsCount);

	return (
		principal && (
			<HStack justifySelf={'flex-end'}>
				<Tooltip label={principal}>
					<>
						{!full &&
					<Text fontSize={'inherit'}>
						{firstChars}...{lastChars}
					</Text>}
						{full &&
					<Text fontSize={'inherit'}>
						{principal}
					</Text>}
					</>

				</Tooltip>
				{!nobtn && <CopyPrincipal text={principal} />}
			</HStack>
		)
	);
};

export default PrincipalFormat;
