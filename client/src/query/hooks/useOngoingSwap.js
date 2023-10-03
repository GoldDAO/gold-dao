import { useCanister, useConnect } from '@connect2ic/react';
import React, { useEffect, useState } from 'react';
import { Principal } from '@dfinity/principal';

const queryOngoingSwaps = async (actor, principal, page) => {
    const ongoingSwaps = await Promise.resolve(
        actor[0].get_ongoing_swaps_by_user({
            page: [page],
            limit: [],
            account: [{ owner: Principal.fromText(principal), subaccount: [] }],
        }),
    );
    return ongoingSwaps;
};

const useOngoingSwaps = (repeat, page) => {
    const [ongoing, setOngoing] = useState();
    const [isLoading, setIsloading] = useState(true);
    const { principal } = useConnect();
    const gldtCoreActor = useCanister('gldtCoreCanister');
    useEffect(() => {
        setIsloading(true);
        if (repeat) {
            setInterval(() => {
                const fetchOngoingSwaps = async () => {
                    await queryOngoingSwaps(gldtCoreActor, principal, page)
                        .then((result) => {
                            setOngoing(result);
                            setIsloading(false);
                        })
                        .catch((error) => {
                            setIsloading(false);
                            console.log('error', error);
                        });
                };
                fetchOngoingSwaps();
            }, 3000);
        } else {
            const fetchOngoingSwaps = async () => {
                await queryOngoingSwaps(gldtCoreActor, principal)
                    .then((result) => {
                        setOngoing(result);
                        setIsloading(false);
                    })
                    .catch((error) => {
                        setIsloading(false);
                        console.log('error', error);
                    });
            };
            fetchOngoingSwaps();
        }
    }, [page]);
    return { ongoing, isLoading };
};

export default useOngoingSwaps;
