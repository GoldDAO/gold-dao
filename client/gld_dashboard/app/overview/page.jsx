'use client';

import OverviewTable from '../../components/Overview/OverviewTable';

export const viewport = {
  themeColor: '#c6c6c6',
};

export default function Overview() {
  return (
    <main className='-mx-5 sm:-mx-0'>
      <OverviewTable />
    </main>
  );
}
