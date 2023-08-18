import React from 'react';
import CustomDialog from './Dialog';
import { SendBatchOffersButton } from '../../../services/commands/SendAllBatchOffers';
import { Box } from '@mui/material';
import Image from 'next/image';

const ConfimDialog = ({ open, setOpen, total }) => {
    return (
        <CustomDialog
            title={`You are in the process to exchange your ${total} g`}
            open={open}
            setOpen={setOpen}
            isButton={false}
        >
            <Box sx={{ display: 'flex', width: '100%', alignItems: 'center', justifyContent: 'space-around' }}>
                <p>{total} g</p>
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    <p>{total * 100} GLDT</p>
                    <Image width={30} height={30} src="/images/gold-ingot.svg" alt="GLDT token icon" />
                </Box>
            </Box>
            <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
                <SendBatchOffersButton />
            </Box>
        </CustomDialog>
    );
};

export default ConfimDialog;