import { CopyIcon } from '@chakra-ui/icons';
import { Button, useToast } from '@chakra-ui/react';
import React from 'react';

const CopyPrincipal = ({ text }) => {
	const toast = useToast({
		position: 'bottom',
	});
	return (
		<Button
			borderRadius={'200px'}
			_hover={{
				bg: 'bg',
			}}
			h={'40px'}
			w={'40px'}
			bg="transparent"
			color="black"
			aria-label="Copy to clipboard"
			onClick={() => {
				navigator.clipboard.writeText(text);
				toast({
					title: 'Success',
					description: 'Principal copied to clipboard',
				});
			}}
		>
			<CopyIcon />
		</Button>
	);
};

export default CopyPrincipal;
