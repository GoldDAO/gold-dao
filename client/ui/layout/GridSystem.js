import { Grid } from '@chakra-ui/react';
import React from 'react';

const GridSystem = ({ children, gap, auto , full}) => {
	return (
		<Grid
			m="0 auto"
			maxW={full ? '100%' : '1540px'}
			templateColumns={auto ? 'repeat(auto-fill, 1fr)' : 'repeat(12, 1fr)'}
			w={'100%'}
			columnGap={gap ? gap : 8}
			rowGap={gap ? gap : [6, 4]}
			wrap="wrap"
			mt={['20px', '20px', 0]}
		>
			{children}
		</Grid>
	);
};

export default GridSystem;
