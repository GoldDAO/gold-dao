import { IconButton, Typography } from '@mui/material';
import React from 'react';
import AutorenewIcon from '@mui/icons-material/Autorenew';
import { useCanister } from '@connect2ic/react';
import { gldNftCanisters } from '@/services/agents';
import { Principal } from '@dfinity/principal';
import { OneK } from '@mui/icons-material';
import { useEffect } from 'react';
import { CustomCircularProgress } from '../styled/common';

const RefreshButton = ({
    token_id,
    g,
    sale_id,
    setStatus,
    setStatusLoading,
    isLoading,
    status,
}) => {
    const actor = useCanister('gldtCoreCanister');

    useEffect(() => {
        setStatusLoading(false);
        refreshStatus();
        const loop = setInterval(() => {
            refreshStatus();
        }, 2500);
        return () => clearInterval(loop);
    }, []);

    const refreshStatus = async () => {
        setStatusLoading(true);
        const res = await actor[0].get_status_of_swap({
            nft_id: token_id,
            gld_nft_canister_id: Principal.fromText(gldNftCanisters[`${g}g`].canisterId),
            sale_id: sale_id,
        });
        if (res) {
            setStatus(Object.keys(res?.Ok?.status[0])[0]);
            setStatusLoading(false);
        }
    };

    return !isLoading ? (
        <IconButton onClick={() => refreshStatus()}>
            <AutorenewIcon />
            <Typography sx={{ fontSize: '16px' }}>{status}</Typography>
        </IconButton>
    ) : (
        <CustomCircularProgress style={{ width: '25px', height: '25px' }} />
    );
};

export default RefreshButton;
