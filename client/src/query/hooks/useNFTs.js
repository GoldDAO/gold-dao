import { gldNftCanisters } from '@/services/agents';
import { Principal } from '@dfinity/principal';
import { useAtom } from 'jotai';
import { useEffect, useState } from 'react';
import { useConnect, useWallet } from '@connect2ic/react';

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
    const { principal } = useConnect();
    const [nfts, setNfts] = useState([]);
    const [isLoading, setLoading] = useState(false);
    useEffect(() => {
        setLoading(true);
        queryNfts(principal, actors)
            .then((result) => {
                getNftWithStatus(result, actors).then((result) => {
                    setNfts(result);
                    setLoading(false);
                });
            })
            .catch((error) => {
                setLoading(false);
            });
    }, []);
    return { nfts, isLoading };
};

const getNftWithStatus = async (nfts, actors) => {
    const weights = Object.keys(gldNftCanisters);
    const res = await Promise.all(
        nfts.map(async (nft, i) => {
            const ind = weights.indexOf(nft.weight + 'g');
            const res = await actors[ind]?.nft_origyn(nft.name);
            return {
                weight: nft.weight,
                name: nft.name,
                status:
                    res?.ok?.current_sale[0]?.sale_type.auction.status.open === null
                        ? res?.ok?.current_sale
                        : undefined,
                sale_id: res.ok?.current_sale[0]?.sale_id,
            };
        }),
    );
    return res;
};

export const useOngoingSwaps = (actors) => {
    const { principal } = useConnect();
    const [isLoading, setLoading] = useState(false);
    const [onSale, setOnSale] = useState([]);
    useEffect(() => {
        if (principal && actors) {
            setLoading(true);
            queryNfts(principal, actors)
                .then((result) => {
                    getOngoingSwapNft(result, actors).then((result) => {
                        setOnSale(result);
                        setLoading(false);
                    });
                })
                .catch((error) => {
                    setLoading(false);
                });
        }
    }, []);

    console.log('useOngoingSwaps', onSale);
    return { onSale, isLoading };
};

const getOngoingSwapNft = async (nfts, actors) => {
    const weights = Object.keys(gldNftCanisters);
    const nftsRes = await Promise.all(
        nfts.map(async (nft, i) => {
            const ind = weights.indexOf(nft.weight + 'g');
            const res = await actors[ind]?.nft_origyn(nft.name);
            if (res?.ok[0]?.current_sale[0]?.sale_type.auction.status.open === null) {
                return {
                    weight: nft.weight,
                    name: nft.name,
                    sale_id: res.ok.current_sale[0].sale_id,
                };
            }
        }),
    );
    console.log('nftsRes', nftsRes);
    return nftsRes;
};
