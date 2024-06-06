import ProposalsTable from '../../components/Proposals/ProposalsTable';

export const viewport = {
  themeColor: '#c6c6c6',
};

export default function Proposals() {
  return (
    <main className='-mx-5 sm:mx-5'>
      <ProposalsTable />
    </main>
  );
}
