import React from 'react';
import CustomDialog from '../Dialog';
import { useAtom } from 'jotai'
import { getTotalCartWeightAtom } from '../../../states/cart';

const ConfimDialog = ({ open, setOpen }) => {
    const [totalWeight] = useAtom(getTotalCartWeightAtom)
    return (
        <CustomDialog title={`You are in the process to exchange your ${totalWeight} g`} open={open} setOpen={setOpen} >
            <p>{totalWeight} g </p> <p>xxx</p>
        </CustomDialog>
    );
};

export default ConfimDialog;