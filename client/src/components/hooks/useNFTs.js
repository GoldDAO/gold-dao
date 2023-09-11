import { gldNftCanisters } from '@/services/agents';
import { emptyOnGoingAtom, updateOnsaleNft, updateOnsaleNftAtom } from '@/states/onSalesNfts';
import { setGetUserAtom } from '@/states/user';
import { Principal } from '@dfinity/principal';
import { useAtom } from 'jotai';
import { useEffect } from 'react';
import { useState } from 'react';

const queryNfts = async (principal, actors) => {
    const weights = Object.keys(gldNftCanisters);
    const nft_promises = actors.map((actor) =>
        actor.balance_of_nft_origyn({
            principal: Principal.fromText(principal),
        }),
    );
    const res = await Promise.all(nft_promises);
    let nfts = [];
    res.forEach((r, i) =>
        nfts.push(...r.ok?.nfts.map((e) => ({ name: e, weight: +weights[i].slice(0, -1) }))),
    );
    return nfts;
};

export const useNft = (actors) => {
    const [user] = useAtom(setGetUserAtom);
    const [nfts, setNfts] = useState([]);
    const [isLoading, setLoading] = useState(false);
    const [, setOnSale] = useAtom(updateOnsaleNftAtom);
    useEffect(() => {
        if (user.principal && actors) {
            setLoading(true);
            queryNfts(user.principal, actors)
                .then((result) => {
                    getNftWithStatus(result, actors, setOnSale).then((result) => {
                        setNfts(result);
                        setLoading(false);
                    });
                })
                .catch((error) => {
                    setLoading(false);
                });
        }
    }, [user]);
    return { nfts, isLoading };
};

const getNftWithStatus = async (nfts, actors, setOnSale) => {
    const weights = Object.keys(gldNftCanisters);
    const res = await Promise.all(
        nfts.map(async (nft, i) => {
            const ind = weights.indexOf(nft.weight + 'g');
            const res = await actors[ind]?.nft_origyn(nft.name);
            if (res?.ok?.current_sale[0]?.sale_type.auction.status.open === null) {
                setOnSale({
                    weight: nft.weight,
                    name: nft.name,
                    status: res?.ok?.current_sale[0]?.sale_type.auction.status.open,
                });
            }
            return {
                weight: nft.weight,
                name: nft.name,
                status:
                    res?.ok?.current_sale[0]?.sale_type.auction.status.open === null
                        ? res?.ok?.current_sale
                        : undefined,
            };
        }),
    );
    return res;
};

export const useOngoingSwaps = (actors) => {
    const [user] = useAtom(setGetUserAtom);
    const [nfts, setNfts] = useState([]);
    const [isLoading, setLoading] = useState(false);
    const [onSale, setOnSale] = useAtom(updateOnsaleNftAtom);
    const [, emptyOnSale] = useAtom(emptyOnGoingAtom);

    useEffect(() => {
        if (user.principal && actors) {
            setLoading(true);
            emptyOnSale();
            queryNfts(user.principal, actors)
                .then((result) => {
                    getOngoingSwapNft(result, actors, setOnSale).then((result) => {
                        setOnSale(result);
                        setLoading(false);
                    });
                })
                .catch((error) => {
                    setLoading(false);
                });
        }
    }, [user]);
    return { onSale, isLoading };
};

const getOngoingSwapNft = async (nfts, actors, setOnSale) => {
    console.log('nftsResult', nfts);
    const weights = Object.keys(gldNftCanisters);

    const res = await Promise.all(
        nfts.map(async (nft, i) => {
            const ind = weights.indexOf(nft.weight + 'g');
            const res = await actors[ind]?.nft_origyn(nft.name);
            if (res?.ok?.current_sale[0]?.sale_type.auction.status.open === null) {
                // swaps.push({
                //     weight: nft.weight,
                //     name: nft.name,
                //     status: res?.ok?.current_sale[0]?.sale_type.auction.status.open,
                // });
                setOnSale({
                    weight: nft.weight,
                    name: nft.name,
                    status: res?.ok?.current_sale[0]?.sale_type.auction.status.open,
                });
            }
        }),
    );
};
