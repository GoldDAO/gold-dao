import { IconButton } from '@mui/material';
import React from 'react';
import AutorenewIcon from '@mui/icons-material/Autorenew';
import { useCanister } from '@connect2ic/react';

const RefreshButton = ({ token_id, g, gldt, status }) => {
    const actor = useCanister('gldtCoreCanister');
    console.log('actor', actor);

    const refreshStatus = async () => {
        // get_status_of_swap(sale_id);
    };
    return (
        <IconButton onClick={refreshStatus}>
            <AutorenewIcon />
        </IconButton>
    );
};

export default RefreshButton;
