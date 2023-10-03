import { useCanister, useWallet } from '@connect2ic/react';
import React, { useState, useEffect } from 'react';
import { Principal } from '@dfinity/principal';

const queryGLDTbalance = async (actor, principal) => {
    const req = await actor[0].icrc1_balance_of({ owner: principal, subaccount: [] });
    return req;
};

const useGLDTbalance = (principal) => {
    const [balance, setBalance] = useState([]);
    const gldtLedgerActor = useCanister('gldtLedgerCanister');
    useEffect(() => {
        if (principal) {
            const fetchBalance = async () => {
                const fetchedBalance = await queryGLDTbalance(
                    gldtLedgerActor,
                    Principal.fromText(principal),
                );
                setBalance((Number(fetchedBalance) / 100000000).toFixed(2));
            };
            fetchBalance();
        }
    }, [principal]);
    return balance;
};

export default useGLDTbalance;
