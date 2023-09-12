import Link from 'next/link';
import React from 'react';
import CheckCircleIcon from '@mui/icons-material/CheckCircle';
import { CustomCircularProgress } from '../../styled/common';
import { Box, Typography } from '@mui/material';
import styled from 'styled-components';

const OnGoing = ({ res }) => {
    return (
        <Box>
            {res ? (
                <>
                    {res.map((e, i) => {
                        return e?.map((el, i) => {
                            if (el.ok) {
                                return (
                                    <>
                                        <Box
                                            sx={{
                                                display: 'flex',
                                                flexDirection: 'column',
                                                alignItems: 'center',
                                                marginBottom: '20px',
                                            }}
                                        >
                                            <CheckCircleIcon />
                                            <Typography>{el.ok?.token_id}</Typography>
                                        </Box>
                                    </>
                                );
                            }
                        });
                    })}
                    <LoaderContainer sx={{ flexDirection: 'column' }}>
                        <Typography>You can follow the process on </Typography>
                        <Link href="/ongoing-swaps">Your ongoing Swap</Link>
                    </LoaderContainer>
                </>
            ) : (
                <LoaderContainer>
                    <CustomCircularProgress />
                    <Typography>Sending Transaction</Typography>
                </LoaderContainer>
            )}
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
