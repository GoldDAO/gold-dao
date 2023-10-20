import {useEffect, useState} from 'react';

const queryTransaction = async (actor, rowsPerPage, currentPage) => {
	// const max = 'getMax';
	// const allTransactionsReq = await Promise.resolve(actor[0]
	// 	.get_transactions({
	// 		start: parseInt(max.log_length) - (rowsPerPage * (currentPage + 1)),
	// 		length: rowsPerPage
	// 	}));
	// const transactions = allTransactionsReq.transactions.reverse();
	// return { transactions, max };
};

export const useTransactionHistory = (rowsPerPage, currentPage) => {
	// const [transactions, setTransactions] = useState([]);
	// const gldtLedgerActor = useCanister('gldtLedgerCanister');
	// useEffect(() => {
	// 	const fetchTransactions = async () => {
	// 		const fetchedTransactions = await queryTransaction(gldtLedgerActor, rowsPerPage, currentPage);
	// 		setTransactions(fetchedTransactions);
	// 	};
	// 	fetchTransactions();
	// }, [rowsPerPage, currentPage]);
	return transactions;
};
