import Image from 'next/image';
import Link from 'next/link';
import React from 'react';
import { Box } from "@mui/system";
import styled from 'styled-components';

const Partners = ({partners}) => {

    return (
        <>
        <SectionTitle>POWERED BY</SectionTitle>
        <PartnersContainer >
            {partners.map((e,i) => (
                    <Link key={i} href={e.url} target="_blank" referrerPolicy="noreferrer" style={{gridColumn: 'span 4', }}>
                        <Image width={200} height={100} src={e.logo} alt={`${e.name}'s logo`} />
                    </Link>
            ))}
            </PartnersContainer>
        </>
    );
};

export default Partners;

export const SectionTitle = styled('h3')`
    font-size: 4.8em;
    color: #D3B872;
    font-weight: 300;
    padding: 26px 0;
`

export const PartnersContainer = styled(Box)`
    display: grid;
    grid-template-columns: repeat(12, 1fr);
    gap: 30px;
    width: 80%; 
    margin: 50px auto;
`