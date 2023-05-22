import { Box } from '@mui/system';
import React from 'react';
import { navigation } from '../../src/content/navigation';
import Link from 'next/link';
import styled from 'styled-components';

const Header = () => {
    return (
        <Box as="header">
            <Box as="ul" >
                {navigation.map((e,i) => (
                    <Box key={i} as="li"><Link href={`${e.path}`}>{e.label}</Link></Box>
                ))}
            </Box>
            <Connect>Connect</Connect>
        </Box>
    );
};

export default Header;

const Connect = styled('button')`
    padding: 5px 15px;
    background-color: #D3B872;
    color: #fff;
`