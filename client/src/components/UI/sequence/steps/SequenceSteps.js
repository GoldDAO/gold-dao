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
import { cartAtom, getCartAtom, getTotalCartWeightAtom } from '@/states/cart';
import OnGoing from './onGoing';
import { theme } from '@/theme/theme';
import MainButton from '../../button/Buttons';
import { CustomCircularProgress } from '../../styled/common';

const SequenceSteps = ({ open, setOpen }) => {
    const [currentStep, setCurrentStep] = useState(0);
    const steps = ['Select NFTs to swap', 'Summary'];
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
            setRes();
        }
    }, [open]);

    const SequenceHeader = () => (
        <SequenceHeaderContainer>
            <Box className="utils-infos">
                <Stepper
                    activeStep={currentStep}
                    sx={{
                        width: '700px',
                    }}
                >
                    {steps.map((label, i) => (
                        <CustomStep key={i}>
                            <StepLabel>{label}</StepLabel>
                        </CustomStep>
                    ))}
                </Stepper>
                <Address address={user.principal} />
            </Box>
        </SequenceHeaderContainer>
    );

    const Controls = ({ disable }) => {
        return (
            <Box
                sx={{
                    width: '100%',
                    paddingTop: '40px',
                    display: 'flex',
                    justifyContent: currentStep > 0 ? 'space-between' : 'flex-end',
                }}
            >
                {currentStep > 0 && (
                    <MainButton secondary={true} action={handlePrev}>
                        Back
                    </MainButton>
                )}
                {currentStep === 1 ? (
                    <SendBatchOffersButton handleNext={handleNext} setRes={setRes} />
                ) : (
                    <MainButton isInactive={disable} action={handleNext}>
                        Next
                    </MainButton>
                )}
            </Box>
        );
    };

    const SelectNfts = ({ open, setOpen, disable }) => {
        const [cart] = useAtom(getCartAtom);
        return (
            <CustomDialog open={open} setOpen={setOpen} title="Exchange your GLDNFT for GLDT">
                <SequenceHeader />
                <MyNfts selectable={true} />
                <Controls disable={cart.length > 0 ? false : true} />
            </CustomDialog>
        );
    };

    const Loader = () => {
        return (
            <Box>
                <CustomCircularProgress />
            </Box>
        );
    };

    const Summary = ({ open, setOpen }) => {
        const [cart] = useAtom(cartAtom);
        const [total] = useAtom(getTotalCartWeightAtom);
        return (
            <CustomDialog open={open} setOpen={setOpen} title="Exchange your GLDNFT for GLDT">
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
            <CustomDialog open={open} setOpen={setOpen} title="Processing Swaps...">
                <OnGoing res={res} />
            </CustomDialog>
        );
    }
};

export default SequenceSteps;

const SequenceHeaderContainer = styled(Box)`
    padding-bottom: 40px;
    display: flex;
    flex-direction: column;
    .utils-infos {
        display: flex;
        .MuiStepper-root {
            padding-right: 30px;
            margin-right: 30px;
            border-right: 1px solid ${theme.colors.gold};
        }
    }
`;

const CustomStep = styled(Step)`
    .MuiStepLabel-root {
        .MuiStepLabel-iconContainer {
            padding-bottom: 10px;
            svg {
                border: 1px solid ${theme.colors.gold};
                border-radius: 20px;
                circle {
                    color: ${theme.colors.grey};
                }
                text {
                    fill: ${theme.colors.gold};
                }
            }
        }
        .css-1u4zpwo-MuiSvgIcon-root-MuiStepIcon-root {
            color: ${theme.colors.gold};
        }
    }
`;
