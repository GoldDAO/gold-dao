import React from 'react';
import CustomDialog from '../Dialog';
import { useAtom } from 'jotai'
import { getTotalCartWeightAtom } from '../../../states/cart';

const ConfimDialog = ({ open, setOpen, total }) => {
    return (
        <CustomDialog title={`You are in the process to exchange your ${total} g`} open={open} setOpen={setOpen} >
            <p>{total} g </p> <p>xxx</p>
        </CustomDialog>
    );
};

export default ConfimDialog;