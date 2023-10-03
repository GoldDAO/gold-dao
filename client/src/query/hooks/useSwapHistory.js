import { useCanister, useConnect } from '@connect2ic/react';
import React, { useEffect, useState } from 'react';
import { Principal } from '@dfinity/principal';

const queryHistory = async (actor, principal, page) => {
    const history = await Promise.resolve(
        actor[0].get_historical_swaps_by_user({
            page: [page],
            limit: [10],
            account: [{ owner: Principal.fromText(principal), subaccount: [] }],
        }),
    );
    return history;
};

const useSwapHistory = (page) => {
    const [history, setHistory] = useState();
    const [isLoading, setIsloading] = useState(true);
    const { principal } = useConnect();
    const gldtCoreActor = useCanister('gldtCoreCanister');
    useEffect(() => {
        setIsloading(true);
        const fetchHistory = async () => {
            await queryHistory(gldtCoreActor, principal, page)
                .then((result) => {
                    setHistory(result);
                    console.log('result', result);
                    setIsloading(false);
                })
                .catch((error) => {
                    setIsloading(false);
                });
        };
        fetchHistory();
    }, [page]);
    return { history, isLoading };
};

export default useSwapHistory;
