import React from 'react';
import Head from 'next/head';
const Metas = ({meta}) => {
	return (
		<Head>
			<title>{meta.title}</title>
			<meta property={'og:title'} content={meta.title} key="title" />
			<meta property={'og:description'} content={meta.description} key="title" />
			<link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
			<link rel="icon" type="image/png" sizes="32x32" href="/favicon/favicon-32x32.png" />
			<link rel="icon" type="image/png" sizes="16x16" href="/favicon/favicon-16x16.png" />
			<meta name="msapplication-TileColor" content="#da532c" />
			<meta name="theme-color" content="#ffffff" />
		</Head>
	);
};

export default Metas;