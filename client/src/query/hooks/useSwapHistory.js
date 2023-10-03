import { useCanister, useConnect } from '@connect2ic/react';
import React, { useEffect, useState } from 'react';
import { Principal } from '@dfinity/principal';

const queryHistory = async (actor, principal, page, limit) => {
    const history = await Promise.resolve(
        actor[0].get_historical_swaps_by_user({
            page: [page],
            limit: [limit],
            account: [{ owner: Principal.fromText(principal), subaccount: [] }],
        }),
    );
    return history;
};

const useSwapHistory = (page, limit) => {
    const [history, setHistory] = useState();
    const [isLoading, setIsloading] = useState(true);
    const { principal } = useConnect();
    const gldtCoreActor = useCanister('gldtCoreCanister');
    useEffect(() => {
        setIsloading(true);
        const fetchHistory = async () => {
            await queryHistory(gldtCoreActor, principal, page, limit)
                .then((result) => {
                    console.log('result', result);
                    setHistory(result);
                    setIsloading(false);
                })
                .catch((error) => {
                    setIsloading(false);
                });
        };
        fetchHistory();
    }, [page, limit]);
    return { history, isLoading };
};

export default useSwapHistory;

export const useMaxEntry = () => {
    const [max, setMax] = useState();
    const [isLoading, setIsloading] = useState(true);
    const { principal } = useConnect();
    const gldtCoreActor = useCanister('gldtCoreCanister');
    useEffect(() => {
        setIsloading(true);
        const fetchHistory = async () => {
            await queryHistory(gldtCoreActor, principal, 1, 10)
                .then((result) => {
                    console.log('result', result);
                    setMax(result.Ok.total);
                    setIsloading(false);
                })
                .catch((error) => {
                    setIsloading(false);
                });
        };
        fetchHistory();
    }, []);
    return { max, isLoading };
};
