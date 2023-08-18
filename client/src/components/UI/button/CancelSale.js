import MainButton from "./Buttons";
import { gldNftCanisters } from "@/services/agents";
import { useAllCanisters } from "@/components/hooks/useAllCanisters";
import { IconButton } from "@mui/material";
import CancelIcon from '@mui/icons-material/Cancel';
import SnackBarFeedback from "@/components/UI/feedback/SnackBar";
import { useState } from "react";

const weights = Object.keys(gldNftCanisters);



export const CancelsaleButton = ({ token_id, weight, setIsOnSale }) => {
    const actors = useAllCanisters()
    const [open, setOpen] = useState(false)

    const handleClose = () => {
        setOpen(false)
    }

    const unlistHandler = async (token_id, weight, actors) => {
        const ind = weights.indexOf(weight + 'g');
        const res = await actors[ind]?.sale_batch_nft_origyn([{ end_sale: token_id }]);
        console.log(res);
        if (res[0].ok) {
            setOpen(true)
            setIsOnSale(false)
        }
    };

    return (
        <>
            <IconButton label="Cancel Sale" onClick={() => unlistHandler(token_id, weight, actors)}>
                <CancelIcon />
            </IconButton>
            <SnackBarFeedback open={open} handleClose={handleClose} text={'Sale Canceled'} />
        </>
    )
}
