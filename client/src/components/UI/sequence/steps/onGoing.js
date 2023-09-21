import { Box, Typography } from '@mui/material';
import styled from 'styled-components';
import dynamic from 'next/dynamic';
import { useState } from 'react';
import { useEffect } from 'react';
import OngoingSwapsTable from '../../table/OngoingSwapsTable';
import { CustomCircularProgress } from '../../styled/common';
import { theme } from '@/theme/theme';

const OnGoing = ({ res }) => {
    return res ? (
        <OngoingSwapsTable res={res} />
    ) : (
        <Box
            sx={{
                gridColumn: 'span 12',
                height: '500px',
                width: '100%',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                flexDirection: 'column',
            }}
        >
            <CustomCircularProgress />
            <Typography
                sx={{ marginTop: '20px', fontStyle: 'italic', color: theme.colors.darkgrey }}
            >
                Awaiting response...
            </Typography>
        </Box>
    );
};

export default OnGoing;

const LoaderContainer = styled(Box)`
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: center;
`;
