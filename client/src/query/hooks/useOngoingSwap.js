import { useCanister, useConnect } from '@connect2ic/react';
import React, { useEffect, useState } from 'react';
import { Principal } from '@dfinity/principal';

const queryOngoingSwaps = async (actor, principal) => {
    const ongoingSwaps = await Promise.resolve(
        actor[0].get_ongoing_swaps_by_user({
            page: [],
            limit: [],
            account: [{ owner: Principal.fromText(principal), subaccount: [] }],
        }),
    );
    return ongoingSwaps;
};

const useOngoingSwaps = (repeat) => {
    const [ongoing, setOngoing] = useState();
    const [isLoading, setIsloading] = useState(true);
    const { principal } = useConnect();
    const gldtCoreActor = useCanister('gldtCoreCanister');
    useEffect(() => {
        setIsloading(true);
        if (repeat) {
            setInterval(() => {
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
            }, 1000);
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
    }, []);
    return { ongoing, isLoading };
};

export default useOngoingSwaps;
