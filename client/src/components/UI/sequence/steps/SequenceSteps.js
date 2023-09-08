import React from 'react';
import CustomDialog from '../Dialog';
import { Button, Stepper, Typography } from '@mui/material';
import MyNfts from '../../table/NftsTable';
import { useState } from 'react';
import { useEffect } from 'react';

const SequenceSteps = ({ open, setOpen }) => {
    console.log('adopen', open);
    const [currentStep, setCurrentSet] = useState(0);
    const handleNext = () => {};

    const SelectNfts = ({ open }) => {
        useEffect(() => {
            console.log('open', open);
        }, [open]);
        return (
            <CustomDialog open={open}>
                <MyNfts />
                <Button onClick={handleNext}>Next</Button>
            </CustomDialog>
        );
    };

    const Summary = () => {
        return (
            <CustomDialog open={open}>
                <p></p>
                <Button onClick={handleNext}>Next</Button>
            </CustomDialog>
        );
    };

    if (currentStep === 0) {
        return <SelectNfts open={open} />;
    } else if (currentStep === 1) {
        return <Summary />;
    }
};

export default SequenceSteps;

const SequenceHeader = () => {
    <Stepper />;
};
