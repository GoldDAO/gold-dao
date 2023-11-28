import React from 'react';
import Grid from './Grid';
import Footer from './Footer';
import { Box } from '@chakra-ui/react';

const Layout = ({ children}) => {

	return (
		<Box sx={{ display: 'flex', flexDirection: 'column' }}>
			<Grid>{children}</Grid>
			<Footer />
		</Box>
	);
};

export default Layout;
