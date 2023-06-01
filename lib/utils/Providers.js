import { Box } from '@mui/system';
import Image from 'next/image';
import React from 'react';
import styled from 'styled-components';

export const providers = [
    {
        name: "Internet Identity",
        url: "https://identity.ic0.app/"
    },
    {
        name: "Plug",
        url: "https://plugwallet.ooo/"
    },
    {
        name: "AstroX ME",
        url: "https://astrox.me/"
    },
    {
        name: "Infinity Wallet",
        url: "https://chrome.google.com/webstore/detail/bitfinity-wallet/jnldfbidonfeldmalbflbmlebbipcnle"
    },
    {
        name: "NFID",
        url: "https://nfid.one/"
    },
    {
        name: "Stoic Wallet",
        url: "https://www.stoicwallet.com/"
    },

]

export const ProviderItem = ({ name, logo }) => {
    return (
        <ProviderItemContainer>
            <ProviderItemName>{name}</ProviderItemName >
            <ProviderItemLogo src={logo} />
        </ProviderItemContainer>
    );
};


const ProviderList = () => {
    return (
        <ProviderListContainer>
            {providers.map((e, i) => (
                <ProviderItem name={e.name} logo={e.logo} key={i} />
            ))}
        </ProviderListContainer>
    );
};

export default ProviderList;


const ProviderItemContainer = styled(Box)`
    display: grid;
    height: 80px;
`

const ProviderItemName = styled(Box)`

`


const ProviderItemLogo = styled(Image)`

`

const ProviderListContainer = styled(Box)`

`