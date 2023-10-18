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
    const [loop, setLoop] = useState(true);
    useEffect(() => {
        setIsloading(true);
        let interval;
        if (loop) {
            interval = setInterval(() => {
                const fetchOngoingSwaps = async () => {
                    await queryOngoingSwaps(gldtCoreActor, principal, page)
                        .then((result) => {
                            setOngoing(result);
                            if (result.Ok.data[0].length < 1) {
                                setLoop(false);
                            }
                            console.log('result.Ok.data[0].length', result.Ok.data[0].length);
                            setIsloading(false);
                        })
                        .catch((error) => {
                            setIsloading(false);
                            console.log('error', error);
                        });
                };
                fetchOngoingSwaps();
            }, 3000);
        } else if (!loop) {
            const fetchOngoingSwaps = async () => {
                await queryOngoingSwaps(gldtCoreActor, principal, page)
                    .then((result) => {
                        setOngoing(result);
                        setIsloading(false);
                        console.log('no repeat');
                    })
                    .catch((error) => {
                        setIsloading(false);
                        console.log('error', error);
                    });
            };
            fetchOngoingSwaps();
        }
        if (interval) {
            return () => clearInterval(interval);
        }
    }, [loop]);
    return { ongoing, isLoading };
};

export default useOngoingSwaps;
