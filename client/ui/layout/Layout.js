import React from 'react';
import Grid from './Grid';
import Footer from './Footer';
import { Box } from '@chakra-ui/react';
import dynamic from 'next/dynamic';
import Header from '@/components/header/Header';

const Layout = ({ children}) => {

	return (
		<Box sx={{ display: 'flex', flexDirection: 'column' }}>
			<Grid>{children}</Grid>
			<Footer />
		</Box>
	);
};

export default Layout;
