import { Label } from '@mui/icons-material';
import { Box, Typography } from '@mui/material';
import React from 'react';
import styled from 'styled-components';
import MainButton from '../button/Buttons';
import { theme } from '@/theme/theme';
import { useDialog } from '@connect2ic/react';
import SequenceSteps from '../sequence/steps/SequenceSteps';
import { useState } from 'react';
import { useAtom } from 'jotai';
import { userAtom } from '@/states/user';

const Banner = () => {
    return (
        <BannerContainer>
            <PageTitle>
                <Typography as="h1" sx={{ fontSize: 'inherit', color: theme.colors.gold }}>
                    GLDT{' '}
                    <Box as="span" style={{ color: theme.colors.black }}>
                        ,
                    </Box>
                </Typography>
                <Typography as="h2" sx={{ fontSize: 'inherit' }}>
                    Token 100% backed by physical gold.
                </Typography>
            </PageTitle>
            <CallToAction />
        </BannerContainer>
    );
};

export default Banner;

const BannerContainer = styled(Box)`
    grid-column: span 12;
    display: grid;
    align-items: center;
    grid-template-columns: repeat(12, 1fr);
`;

const PageTitle = styled(Box)`
    height: fit-content;
    font-size: 64px;
    position: relative;
    grid-column: span 6;
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    h1,
    h2 {
        position: relative;
        z-index: 2;
        grid-column: 2/7;
        line-height: 1.2em;
        span {
            position: relative;
            left: -10px;
        }
    }
    ::after {
        content: '';
        display: block;
        position: absolute;
        background-image: url(/images/banner/ingot.svg);
        background-repeat: no-repeat;
        background-size: 96%;
        background-position: center;
        top: -150px;
        width: 500px;
        left: 100px;
        height: 500px;
    }
    ::before {
        content: '';
        display: block;
        position: absolute;
        background-image: url(/images/banner/small_arrow.svg);
        background-repeat: no-repeat;
        background-size: 96%;
        background-position: center;
        width: 300px;
        height: 300px;
        left: 480px;
        top: -200px;
    }
`;

const CallToAction = () => {
    const { open } = useDialog();
    const [isOpen, setIsOpen] = useState(false);
    const [user] = useAtom(userAtom);

    return (
        <CallToActionContainer>
            <Circle>
                <Typography as="h3">
                    Swap your <Name>GLDNFT</Name> for <Name>GLDT</Name>
                </Typography>
                <MainButton
                    action={() => {
                        user.isConnected ? setIsOpen(!isOpen) : open();
                    }}
                >
                    Start swapping my
                    <Name style={{ color: theme.colors.white }}>&nbsp;GLDNFTs</Name>
                </MainButton>
                <SequenceSteps open={isOpen} setOpen={setIsOpen} />
            </Circle>
        </CallToActionContainer>
    );
};
const Circle = styled(Box)`
    background-image: url(/images/banner/circle.svg);
    background-repeat: no-repeat;
    background-size: 96%;
    background-position: center;
    width: 400px;
    height: 400px;
    display: flex;
    position: relative;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    ::after {
        content: '';
        display: block;
        position: absolute;
        background-image: url(/images/banner/arrow.svg);
        background-repeat: no-repeat;
        background-size: 96%;
        background-position: center;
        left: -300px;
        top: 90px;
        width: 400px;
        height: 400px;
    }
`;
const CallToActionContainer = styled(Box)`
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    grid-column: span 5;
`;

const Name = styled(Box)`
    font-style: italic;
    display: inline;
    font-weight: 600;
    color: ${theme.colors.gold};
`;
