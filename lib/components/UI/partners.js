import Image from 'next/image';
import Link from 'next/link';
import React from 'react';
import { Box } from "@mui/system";
import styled from 'styled-components';

const Partners = ({ partners }) => {

    return (
        <>
            <SectionTitle>POWERED BY</SectionTitle>
            <PartnersContainer >
                {partners.map((e, i) => (
                    <Link key={i} href={e.url} target="_blank" referrerPolicy="noreferrer" style={{ gridColumn: 'span 4', }}>
                        <Image width={e.width} height={100} src={e.logo} alt={`${e.name}'s logo`} />
                    </Link>
                ))}
            </PartnersContainer>
        </>
    );
};

export default Partners;

export const SectionTitle = styled('h3')`
    color: #333;
    font-weight: 300;
    padding-bottom: 15px;
    border-bottom: 1px solid #D3B872;
    font-size: 2em;
    padding: 80px 0 15px 0;
    margin-bottom: 80px;
`

export const PartnersContainer = styled(Box)`
    display: grid;
    grid-template-columns: repeat(12, 1fr);
    gap: 30px;
    width: 80%; 
    margin: 50px auto;
    @media (max-width: 1140px){
        grid-template-columns: repeat(12, 1fr);
    }
    @media (max-width: 840px){
        grid-template-columns: repeat(8, 1fr);
    }
    @media (max-width: 480px){
        grid-template-columns: repeat(3, 1fr);
    }
`