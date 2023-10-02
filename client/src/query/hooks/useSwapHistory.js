import { useCanister, useConnect } from '@connect2ic/react';
import React, { useEffect, useState } from 'react';
import { Principal } from '@dfinity/principal';

const queryHistory = async (actor, principal) => {
    const history = await Promise.resolve(
        actor[0].get_historical_swaps_by_user({
            page: [],
            limit: [],
            account: [{ owner: Principal.fromText(principal), subaccount: [] }],
        }),
    );
    return history;
};

const useSwapHistory = () => {
    const [history, setHistory] = useState();
    const [isLoading, setIsloading] = useState(true);
    const { principal } = useConnect();
    const gldtCoreActor = useCanister('gldtCoreCanister');
    useEffect(() => {
        setIsloading(true);
        const fetchHistory = async () => {
            await queryHistory(gldtCoreActor, principal)
                .then((result) => {
                    setHistory(result);
                    console.log('historyResult', result);
                    setIsloading(false);
                })
                .catch((error) => {
                    setIsloading(false);
                });
        };
        fetchHistory();
    }, []);
    return { history, isLoading };
};

export default useSwapHistory;
