'use client';

import { useEffect, useState } from 'react';

import Image from 'next/image';
import Chart from './Chart';
import { GLDGovIcon } from '../../utils/svgs';
import Modal from '../shared/modal/modal';
import ModalChartMobile from '../shared/modal/modal-chart-mobile';
import { data1 } from '../../utils/datas';
import { parseNumbers } from '../../utils/parsers';
import useServices from '../../hooks/useServices';
import useCharts from '../../hooks/useCharts';

export default function Graphs() {
  const [selectedTab, setSelectedTab] = useState('Treasury');
  const { getSupplyChart, getTreasuryChart, gldGovTreasury } = useServices();
  const {
    stakersData, holdersData, burnData, gldGovSupply, setLiquidChartData, liquidData
  } = useCharts();
  const [amount, setAmount] = useState();
  const [infoModal, setInfoModal] = useState(null);

  const tabs = ['Treasury', 'Staked', 'Liquid', 'Burned', 'Holders'];

  const handleTabClick = (tab) => setSelectedTab(tab);
  // const [isMobile, setIsMobile] = useState(null);
  useEffect(() => {
    getSupplyChart();
    getTreasuryChart();
  }, [selectedTab]);

  const deriveLiquidData = (gldgovSupply, staked) => {
    const liquid = gldgovSupply.map(({ label: supplyLabel, value }) => {
      const stakedValue = staked.find(
        ({ label: stakedLabel }) => stakedLabel === supplyLabel,
      );
      return {
        label: supplyLabel,
        value: value - (stakedValue?.value ?? 0),
      };
    });

    setLiquidChartData(liquid);
  };

  useEffect(() => {
    deriveLiquidData(gldGovSupply.data, stakersData.data);
  }, [stakersData?.data, stakersData?.data.length, stakersData.loading,
    gldGovSupply?.data, gldGovSupply?.data.length, gldGovSupply.loading]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        if (selectedTab === 'Treasury') {
          const result = await gldGovTreasury();
          setAmount(result);
        }
        if (selectedTab === 'Staked') {
          setAmount(stakersData.data[stakersData.data.length - 1].value);
        }
        if (selectedTab === 'Holders') {
          setAmount(holdersData.data[holdersData.data.length - 1].value);
        }
        if (selectedTab === 'Burned') {
          setAmount(burnData.data[burnData.data.length - 1].value);
        }
        if (selectedTab === 'Liquid') {
          setAmount(liquidData.data[liquidData.data.length - 1].value)
        }
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    fetchData();
  }, [selectedTab]);

  const displayAmount = parseNumbers(amount);

  return (
    <>
      <article className="flex flex-row w-full mt-2 border-[0.5px] border-DarkGrey bg-SoftGrey rounded-4xl shadow-[0px 0px 12px 0px #00000026]">
        <div className="flex flex-col sm:border-r-2 w-full sm:w-[33%]">
          {tabs.map((tab, index) => (
            <span
              key={`${tab}-${index}`}
              className={`sm:focus:bg-Gold flex-1 border-t-2 px-3 sm:px-5 text-left h-17 flex justify-between sm:justify-start items-center w-full 
              ${index === 0 ? ' rounded-t-[36px] sm:rounded-tr-none' : ''} 
              ${index === tabs.length - 1 ? 'rounded-b-[36px] sm:rounded-br-none' : ''}
              ${tab === selectedTab ? 'sm:bg-Gold sm:border-t-2 sm:border-r-2 sm:border-Gold sm:text-white sm:font-extrabold' : ''} py-4`}
              onClick={() => {
                if (window.matchMedia('(max-width: 768px)').matches) {
                  document.getElementById('chartmodalgraph').showModal();
                }
                setInfoModal({
                  title: selectedTab,
                  image: 'image',
                  info: 'info',
                  amount,
                  data: data1,
                });
                handleTabClick(tab);
              }}
            >
              <div className="flex w-[60%] justify-between items-center">
                <h3 className="max-md:text-xs pl-2 sm:pl-5 max-md:font-bold">{tab}</h3>
                <Image
                  src={'/svg/chartIcon.svg'}
                  alt=""
                  width={72}
                  height={36}
                  className="sm:hidden w-[72px] h-[36px] object-cover"
                />
              </div>
              <div className="flex justify-end min-w-[120px] sm:hidden items-center gap-1">
                <h5 className="font-bold text-xs">
                  {tab !== 'Treasury' ? 'N/A' : parseNumbers(amount)}
                </h5>
                <GLDGovIcon />
              </div>
            </span>
          ))}
        </div>
        <Chart name={selectedTab} amount={displayAmount} />
      </article>
      <Modal title={`chart${selectedTab}`} idModal="chartmodalgraph" amount={infoModal?.amount}>
        <ModalChartMobile name={selectedTab} />
      </Modal>
    </>
  );
}
