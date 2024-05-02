import TransactionsTable from '../../components/Transactions/TransactionsTable.jsx';

export const viewport = {
  themeColor: '#c6c6c6',
};

export default function Transactions() {
  return (
    <main className='-mx-5'>
      <TransactionsTable />
    </main>
  );
}
