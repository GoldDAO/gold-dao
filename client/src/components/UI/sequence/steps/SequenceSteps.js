import React from 'react';
import CustomDialog from '../Dialog';
import { Box, Button, CircularProgress, Step, StepLabel, Stepper, Typography } from '@mui/material';
import MyNfts from '../../table/NftsTable';
import { useState } from 'react';
import { useEffect } from 'react';
import Address from '../../wallet/Address';
import { useAtom } from 'jotai';
import { userAtom } from '@/states/user';
import styled from 'styled-components';
import { SendBatchOffersButton } from '../../button/SendAllBatchOffers';
import SummaryTable from './SummaryTable';
import { cartAtom, getTotalCartWeightAtom } from '@/states/cart';
import OnGoing from './onGoing';

const SequenceSteps = ({ open, setOpen }) => {
    const [currentStep, setCurrentStep] = useState(0);
    const steps = ['Select NFTs to swap', 'Summary', 'Transaction Sent'];
    const [user] = useAtom(userAtom);
    const [res, setRes] = useState();

    const handleNext = () => {
        setCurrentStep((prevStep) => prevStep + 1);
    };
    const handlePrev = () => {
        setCurrentStep((prevStep) => prevStep - 1);
    };

    useEffect(() => {
        if (!open) {
            setCurrentStep(0);
        }
    }, [open]);

    const SequenceHeader = () => (
        <SequenceHeaderContainer>
            <Stepper
                activeStep={currentStep}
                sx={{
                    width: '70%',
                }}
            >
                {steps.map((label, i) => (
                    <Step>
                        <StepLabel>{label}</StepLabel>
                    </Step>
                ))}
            </Stepper>
            <Address address={user.principal} />
        </SequenceHeaderContainer>
    );

    const Controls = () => {
        return (
            <Box>
                {currentStep > 0 && <Button onClick={handlePrev}>Back</Button>}
                {currentStep === 1 ? (
                    <SendBatchOffersButton handleNext={handleNext} setRes={setRes} />
                ) : (
                    <Button onClick={handleNext}>Next</Button>
                )}
            </Box>
        );
    };

    const SelectNfts = ({ open, setOpen }) => {
        return (
            <CustomDialog open={open} setOpen={setOpen}>
                <SequenceHeader />
                <MyNfts selectable={true} />
                <Controls />
            </CustomDialog>
        );
    };

    const Loader = () => {
        return (
            <Box>
                <CircularProgress />
            </Box>
        );
    };

    const Summary = ({ open, setOpen }) => {
        const [cart] = useAtom(cartAtom);
        const [total] = useAtom(getTotalCartWeightAtom);
        return (
            <CustomDialog open={open} setOpen={setOpen}>
                <SequenceHeader />
                <SummaryTable g={total} nfts={cart} />
                <Controls />
            </CustomDialog>
        );
    };

    if (currentStep === 0) {
        return <SelectNfts open={open} setOpen={setOpen} />;
    } else if (currentStep === 1) {
        return <Summary open={open} setOpen={setOpen} />;
    } else if (currentStep === 2) {
        return (
            <CustomDialog open={open} setOpen={setOpen}>
                <OnGoing res={res} />
            </CustomDialog>
        );
    }
};

export default SequenceSteps;

const SequenceHeaderContainer = styled(Box)`
    display: flex;
    justify-content: space-between;
`;
