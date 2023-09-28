import dynamic from 'next/dynamic';
import React from 'react';
import Logo from '/public/images/logo.svg';
import Image from 'next/image';
import C2icButton from '../../c2ic/C2icButton';
import { useConnect } from '@connect2ic/react';
import Balance from './Balance';
import Principal from './Principal';
import Link from 'next/link';
import { Grid, GridItem } from '@chakra-ui/react';

const Header = () => {
    const { isConnected } = useConnect();
    return (
        <header
            style={{
                gridColumn: 'span 12',
            }}
        >
            <Grid p={5} gridTemplateColumns={'repeat(12, 1fr)'}>
                <GridItem gridColumn={'span 2'}>
                    <Image src={Logo} width={50} height={50} />
                </GridItem>
                <GridItem gridColumn={'span 6'}>
                    {isConnected && (
                        <Grid gridTemplateColumns={'repeat(12, 1fr)'}>
                            <GridItem gridColumn={'span 6'}>
                                <Link href="/my-account">My Account</Link>
                            </GridItem>
                            <GridItem gridColumn={'span 3'}>
                                <Principal />
                            </GridItem>
                            <GridItem gridColumn={'span 3'}>
                                <Balance />
                            </GridItem>
                        </Grid>
                    )}
                </GridItem>
                <GridItem gridColumn={'12/13'}>
                    <C2icButton />
                </GridItem>
            </Grid>
        </header>
    );
};

export default Header;
