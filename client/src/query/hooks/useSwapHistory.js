import { useCanister, useConnect } from '@connect2ic/react';
import React, { useEffect, useState } from 'react';
import { Principal } from '@dfinity/principal';

const queryHistory = async (actor, principal) => {
    const history = await Promise.resolve(
        actor[0].get_swaps_by_user({
            page: [],
            limit: [],
            account: [{ owner: Principal.fromText(principal), subaccount: [] }],
        }),
    );
    return history;
};

const useSwapHistory = () => {
    const [history, setHistory] = useState([]);
    const { principal } = useConnect();
    const gldtCoreActor = useCanister('gldtCoreCanister');
    useEffect(() => {
        if (principal) {
            const fetchHistory = async () => {
                const fetchedHistory = await queryHistory(gldtCoreActor, principal);
                setHistory(fetchedHistory);
            };
            fetchHistory();
        }
    }, [principal]);
    return history;
};

export default useSwapHistory;
