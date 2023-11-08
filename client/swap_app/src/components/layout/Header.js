import WithSubnavigation from '@ui/header/Navigation';
import { Box, Grid, GridItem } from '@chakra-ui/react';
import Principal from '@ui/principal/Principal';
import Balance from '@ui/principal/Balance';
import { useConnect } from '@connect2ic/react';

const NAV_ITEMS = [
    {
        label: 'My Account',
        href: '/my-account',
    },
];

const Header = () => {
    const { isConnected, principal } = useConnect();
    return (
        <Box
            as="header"
            style={{
                gridColumn: 'span 12',
            }}
            px={['20px', '20px', '40px']}
        >
            <WithSubnavigation nav={NAV_ITEMS}>
                <Wallet />
            </WithSubnavigation>
        </Box>
    );
};

const Wallet = () => {
    return (
        <Grid>
            <GridItem gridColumn={'span 3'}>
                <Principal />
            </GridItem>
            <GridItem gridColumn={'span 3'}>
                <Balance />
            </GridItem>
        </Grid>
    );
};
export default Header;
