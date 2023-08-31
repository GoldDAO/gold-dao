import { useState, useEffect } from "react";
import { gldNftCanisters } from "@/services/agents";
import { useAllCanisters } from "@/components/hooks/useAllCanisters";
import { CircularProgress, IconButton } from "@mui/material";
import CancelIcon from '@mui/icons-material/Cancel';
import SnackBarFeedback from "@/components/UI/feedback/SnackBar";
import styled from "styled-components";


export const CancelsaleButton = ({ token_id, weight, setIsOnSale, setFeedback }) => {
    const actors = useAllCanisters();

    const [isLoading, setIsLoading] = useState(false);

    const weights = Object.keys(gldNftCanisters);

    const unlistHandler = async (token_id, weight, actors) => {
        const ind = weights.indexOf(weight + 'g');
        setIsLoading(true)
        const res = await actors[ind]?.sale_batch_nft_origyn([{ end_sale: token_id }]);
        console.log(res);
        if (res[0].ok) {
            successFeedback();
        }
    };

    const successFeedback = () => {
        setFeedback(true);
        setIsOnSale(false);
        setIsLoading(false);
    }

    return (
        <>
            {isLoading ?
                <SmallCircularProgress />
                :
                <IconButton label="Cancel Sale" onClick={() => unlistHandler(token_id, weight, actors)}>
                    <CancelIcon />
                </IconButton>
            }
        </>
    )
}

const SmallCircularProgress = styled(CircularProgress)`
    width: 25px !important;
    height: 25px !important;
    margin-left: 10px;
`