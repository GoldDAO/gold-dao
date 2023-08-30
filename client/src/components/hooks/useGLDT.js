import { useCanister } from '@connect2ic/react';
import React, { useState, useEffect } from 'react';

const queryTransaction = async (actor, rowsPerPage, currentPage) => {
    const max = await actor[0].get_transactions({ start: 1, length: 2 })
    const allTransactionsReq = await Promise.resolve(actor[0].get_transactions({ start: currentPage * rowsPerPage, length: rowsPerPage }))
    const transactions = allTransactionsReq.transactions
    return { transactions, max }
}

export const useGldtLedgerTransactions = (rowsPerPage, currentPage) => {
    console.log('rowsPerPage, currentPage', rowsPerPage, currentPage)
    const [transactions, setTransactions] = useState([]);
    const gldtLedgerActor = useCanister('gldtLedgerCanister')


    useEffect(() => {
        console.log('transactions', transactions)
    }, [transactions])
    useEffect(() => {
        const fetchTransactions = async () => {
            const fetchedTransactions = await queryTransaction(gldtLedgerActor, rowsPerPage, currentPage);
            setTransactions(fetchedTransactions);
        };
        fetchTransactions();
    }, [rowsPerPage, currentPage]);
    return transactions
};
