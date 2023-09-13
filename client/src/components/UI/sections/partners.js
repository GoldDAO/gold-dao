import Image from 'next/image';
import Link from 'next/link';
import React from 'react';
import { Box } from '@mui/system';
import styled from 'styled-components';
import { SectionTitle } from './../styled/common';
import { Typography } from '@mui/material';

const Partners = ({ partners }) => {
    return (
        <PartnerSection>
            <Typography as="h4">Powered by</Typography>
            <PartnersContainer>
                {partners.map((e, i) => (
                    <Link key={i} href={e.url} target="_blank" referrerPolicy="noreferrer">
                        <Image width={e.width} height={100} src={e.logo} alt={`${e.name}'s logo`} />
                    </Link>
                ))}
            </PartnersContainer>
        </PartnerSection>
    );
};

export default Partners;

export const PartnersContainer = styled(Box)`
    display: grid;
    grid-column: 4/13;
    grid-template-columns: repeat(10, 1fr);
    a {
        grid-column: span 2;
        display: flex;
        align-items: center;
        justify-content: center;
        img {
            max-width: 150px;
        }
    }
    @media (max-width: 1140px) {
        grid-template-columns: repeat(12, 1fr);
    }
    @media (max-width: 840px) {
        grid-template-columns: repeat(8, 1fr);
    }
    @media (max-width: 480px) {
        grid-template-columns: repeat(3, 1fr);
    }
`;

const PartnerSection = styled(Box)`
    grid-column: 2/12;
    display: grid;
    grid-template-columns: repeat(10, 1fr);
    h4 {
        font-size: 24px;
        grid-column: 1/3;
    }
`;
