'use client';

import { useEffect } from 'react';
import CollapseContainer from '../components/Home/Collapse/CollapseContainer';
import MobileRedirectButtons from '../components/Home/MobileRedirectButtons';
import GLDGovInfo from '../components/Home/GLDGovInfo';
import GraphCard from '../components/Home/GraphCard';
import Neurons from '../components/Home/Neurons';
import StakingReturn from '../components/Home/StakingReturn';
import TradeOn from '../components/Home/TradeOn';
import useTokenMetrics from '../hooks/useTokenMetrics';
import useSuperStats from '../hooks/useSuperStats';
import { fetchBurnData } from '../services/icpApi';
import useCharts from '../hooks/useCharts';
import useServices from '../hooks/useServices';

export const viewport = {
  themeColor: '#c6c6c6',
};

export default function Home() {
  // edit navigation bar color
  const { getStakedAmount } = useTokenMetrics();
  const {
    getHoldersData, getRewardPool, getReservePool, getSNSFundCanister,
  } = useSuperStats();
  const { setBurnData } = useCharts();
  fetchBurnData();
  const { getTreasuryChart } = useServices();

  useEffect(() => {
    let metaTag = document.querySelector('meta[name="theme-color"]');

    if (!metaTag) {
      metaTag = document.createElement('meta');
      metaTag.setAttribute('name', 'theme-color');
      document.getElementsByTagName('head')[0].appendChild(metaTag);
    }

    metaTag.setAttribute('content', '#c6c6c6');
  }, []);

  // getters
  useEffect(() => {
    (async () => {
      const [burnData] = await Promise.allSettled([
        fetchBurnData(),
        getStakedAmount(),
        getHoldersData(),
        getRewardPool(),
        getReservePool(),
        getSNSFundCanister(),
        getTreasuryChart(),
      ]);

      setBurnData(burnData.value ?? []);
    })();
  }, []);

  return (
    <main className="flex flex-col justify-center w-full sm:pl-[30px] sm:pr-[18px]">
      <GLDGovInfo />
      <GraphCard />
      <StakingReturn />
      <Neurons />
      <CollapseContainer className='hidden sm:flex' />
      <MobileRedirectButtons className='sm:hidden' />
      <TradeOn />
    </main>
  );
}
