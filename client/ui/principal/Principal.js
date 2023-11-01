import React, { useEffect } from 'react';
import { HStack, Text, Tooltip } from '@chakra-ui/react';
import CopyPrincipal from '@ui/gldt/CopyPrincipal';

const PrincipalFormat = ({ principal }) => {
	const charsCount = 3;
	const firstChars = principal?.slice(0, charsCount);
	const lastChars = principal?.substring(principal.length - charsCount);

	return (
		principal && (
			<HStack justifySelf={'flex-end'}>
				<Tooltip label={principal}>
					<Text>
						{firstChars}...{lastChars}
					</Text>
				</Tooltip>
				<CopyPrincipal text={principal} />
			</HStack>
		)
	);
};

export default PrincipalFormat;
