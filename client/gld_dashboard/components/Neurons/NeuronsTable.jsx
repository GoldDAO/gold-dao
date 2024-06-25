/* eslint-disable max-len */
/* eslint-disable no-nested-ternary */

'use client';

/* eslint-disable no-console */

import {
  Divider,
  Tooltip,
  Button,
  Pagination,
  PaginationItemType,
  Accordion,
  AccordionItem,
  Select,
  SelectItem,
  Spinner,
  Table,
  TableBody,
  TableRow,
  TableCell,
  TableHeader,
  TableColumn,
} from '@nextui-org/react';
import { useEffect, useState } from 'react';
import { Bounce, toast } from 'react-toastify';
import {
  ChevronDownArrow, ChevronRightArrow, InfoIcon, SimpleDownArrow,
} from '../../utils/svgs';
import { copyContent, elapsedTime, formatDateFromSeconds } from '../../utils/functions';
import usePagination from '../../hooks/usePagination';
import { parseNumbers } from '../../utils/parsers';
import useActor from '../../hooks/useActor';

const URL = 'https://sns-api.internetcomputer.org/api/v1/snses/tw2vt-hqaaa-aaaaq-aab6a-cai/neurons';

const NeuronsTable = () => {
  const [neurons, setNeurons] = useState({ loading: true, data: [] });
  const {
    limit, setLimit, page, setPage, setItemsAmount, totalPages,
  } = usePagination();
  const [governance] = useActor('governance');
  const [systemParams, setSystemParams] = useState({});

  const [copyState, setCopyState] = useState(false);

  useEffect(() => {
    if (copyState) {
      toast.success('Copied', {
        position: 'top-right',
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      setCopyState(false);
    }
  }, [copyState]);

  useEffect(() => {
    window.scrollTo(0, 0);
    setNeurons({ loading: true, data: [] });
    (async () => {
      try {
        const res = await fetch(
          `${URL}?offset=${(page - 1) * limit}&limit=${limit}&sort_by=-created_timestamp_seconds`,
        );
        const data = await res.json();
        setItemsAmount(data?.total_neurons || 0);
        setNeurons({ loading: false, data: data?.data });
      } catch (err) {
        console.log('get neurons fetch error:', err);
        setNeurons({ loading: false, data: [] });
      }
    })();

    // eslint-disable-next-line no-unused-expressions
    (async () => {
      try {
        const res = await governance.get_nervous_system_parameters();
        const data = {
          max_neuron_age_for_age_bonus: Number(res.max_neuron_age_for_age_bonus[0]),
          max_age_bonus_percentage: Number(res.max_age_bonus_percentage[0]),
          max_dissolve_delay_seconds: Number(res.max_dissolve_delay_seconds[0]),
          max_dissolve_delay_bonus_percentage: Number(res.max_dissolve_delay_bonus_percentage[0]),
        };
        setSystemParams(data);
      } catch (err) {
        console.log('systemParams error', err);
        setSystemParams({});
      }
    })();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [limit, page]);

  const tooltipClassName = {
    base: ['max-w-[400px]'],
    content: ['py-2 px-4 shadow-xl', 'text-white bg-black'],
  };

  const renderItem = ({
    ref,
    key,
    value,
    isActive,
    activePage,
    onNext,
    onPrevious,
    // eslint-disable-next-line no-shadow
    setPage,
    className,
  }) => {
    if (value === PaginationItemType.NEXT) {
      return (
        <Button
          key={key}
          isIconOnly
          radius="full"
          size="sm"
          className="bg-transparent"
          onClick={onNext}
        >
          <ChevronRightArrow />
        </Button>
      );
    }
    if (value === PaginationItemType.PREV && activePage !== 1) {
      return (
        <Button
          key={key}
          isIconOnly
          radius="full"
          size="sm"
          className="bg-transparent rotate-180"
          onClick={onPrevious}
        >
          <ChevronRightArrow />
        </Button>
      );
    }
    if (value === PaginationItemType.PREV && activePage === 1) {
      return null;
    }

    if (value === PaginationItemType.DOTS) {
      return (
        <Button key={key} className={className}>
          ...
        </Button>
      );
    }

    // cursor is the default item
    return (
      <Button
        isIconOnly
        radius="full"
        size="sm"
        key={key}
        ref={ref}
        className={`text-xs font-normal ${isActive ? 'text-white bg-[#D9D9D9] font-bold' : 'bg-transparent'}`}
        onClick={() => setPage(value)}
      >
        {value}
      </Button>
    );
  };

  const handleSelectChange = (e) => {
    setLimit(parseInt(e.target.value, 10));
    setPage(1);
  };

  return (
    <Table
      shadow="none"
      isHeaderSticky={true}
      hideScrollbar={true}
      radius="none"
      className="w-full sm:mt-10 sm:mb-44 overflow-hidden scrollbar-thin"
      classNames={{
        base: ' overflow-scroll',
        table: 'min-h-fit p-0',
        wrapper: 'p-0 border-1 border-DarkGrey sm:rounded-4xl bg-CardBackground',
        thead: [
          '[&>tr]:shadow-none [&>tr]:first:rounded-none border-DarkGrey',
          '[&>tr:last-child]:!mt-0',
          'h-fit',
          'shadow-none',
        ],
        th: [
          'bg-CardBackground',
          'shadow-none',
          'text-Gold',
          'cursor-default',
          'text-base',
          'h-40',
          'px-0',
        ],
        tr: [
          'border-b-1',
          'border-DarkGrey',
          'h-20',
          'data-[first=true]:border-b-1',
          'group-data-[odd=true]:first:before:rounded-none',
        ],
        td: ['group-data-[middle=true]:before:rounded-none', 'text-Taupe', 'text-base'],
      }}
    >
      <TableHeader>
        <TableColumn>
          <div className='flex justify-between mx-10'>
            <h1 className='text-xl font-bold text-start text-black mb-2'>Neurons</h1>
            <div className='hidden sm:flex gap-x-8 items-center'>
              <Pagination
                disableCursorAnimation
                showControls
                total={totalPages}
                page={page}
                initialPage={1}
                className="gap-2"
                radius="full"
                renderItem={renderItem}
                variant="light"
                onChange={(val) => setPage(val)}
              />
              <Select
                label="Rows per page"
                radius='full'
                variant="bordered"
                labelPlacement={'outside-left'}
                selectedKeys={[limit]}
                onChange={(e) => handleSelectChange(e)}
                className="w-48 items-center"
                selectorIcon={<ChevronDownArrow />}
                classNames={{
                  value: 'font-bold text-white',
                  popoverContent: '-mt-1 rounded-lg',
                  trigger: 'bg-[#D9D9D9] border-none min-h-fit h-8',
                  selectorIcon: 'text-black',
                }}
                listboxProps={{
                  itemClasses: {
                    title: 'font-bold',
                    base: [
                      'rounded-full',
                      'transition-opacity',
                      'data-[hover=true]:text-foreground',
                      'data-[hover=true]:bg-default-100',
                      'dark:data-[hover=true]:bg-default-50',
                      'data-[selectable=true]:focus:bg-default-50',
                      'data-[pressed=true]:opacity-70',
                      'data-[focus-visible=true]:ring-default-500',
                    ],
                  },
                }}
              >
                <SelectItem key={10} value={10}>
                  10
                </SelectItem>
                <SelectItem key={50} value={50}>
                  50
                </SelectItem>
                <SelectItem key={100} value={100}>
                  100
                </SelectItem>
              </Select>
            </div>
          </div>
          <Divider className='my-2' />
          <div className='flex justify-between text-start items-center mt-6 mx-6 gap-x-8'>
            <h2 className="flex items-center w-[150px] gap-x-2 h-12">
              ID
              <div className='hidden sm:block'>
                <Tooltip
                  content='The unique identifier of the neuron.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center w-[120px] gap-x-2 h-12">
              State
              <div className='hidden sm:block'>
                <Tooltip
                  content='The state of the neuron.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center w-[155px] gap-x-2 h-12">
              Staked GLDGov
              <div className='hidden sm:block'>
                <Tooltip
                  content='The number of GLDGov tokens staked in the neuron. The dissolve delay bonus and age bonus are applied to this value plus staked maturity in order to calculate the voting power of the neuron.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center w-[80px] gap-x-2 h-12">
              Maturity
              <div className='hidden sm:block'>
                <Tooltip
                  content='The total accumulated maturity of the neuron, regular and staked.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center w-[120px] gap-x-2 h-12">
              Dissolve Delay
              <div className='hidden sm:block'>
                <Tooltip
                  content='The minimum time period over which the neuron owner locks up their staked GLDGov tokens. This determines how long it will take to dissolve if the neuron is placed into the Dissolving state. Once a neuron has been placed into the Dissolving state, its dissolve delay falls over the passage of time, rather like a kitchen timer, until either it is stopped or it reaches zero. When it reaches zero and enters the Dissolved state, its owner can perform a final disburse action to unlock the balance of GLDGov tokens. The dissolve delay can be configured up to a maximum of 2 years, and must be 91 days or greater for a neuron to be able to vote and earn voting rewards.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center justify-center w-[75px] gap-x-2 h-12 mr-[2%]">
              Age
              <div className='hidden sm:block'>
                <Tooltip
                  content='The period of time that has elapsed since the neuron was created or last entered the Not Dissolving state. While dissolving, a neuron’s age is considered zero. Increasing the stake of a neuron will decrease its age. For example, if the stake is doubled, the age will be halved. Splitting a neuron creates a child neuron that inherits the age of its parent.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center w-1/12 gap-x-2 h-12 mr-10">
              Voting Power
              <div className='hidden sm:block'>
                <Tooltip
                  content='The voting power of the neuron.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
          </div>
        </TableColumn>
      </TableHeader>
      <TableBody
        isLoading={neurons.loading}
        loadingContent={
          <Spinner
            className="mt-28"
            classNames={{ circle1: 'border-b-Beige', circle2: 'border-b-Beige' }}
          />
        }
      >
        {neurons.data.map((neuron) => {
          const dissolveDelayBonus = neuron.state?.split(/(?=[A-Z])/)
            .join(' ') === 'Dissolved' ? '-' : neuron.dissolve_state?.DissolveDelaySeconds ? (((neuron.dissolve_state?.DissolveDelaySeconds || 1) / ((systemParams.max_dissolve_delay_seconds || 1) * (systemParams.max_dissolve_delay_bonus_percentage || 1))) * 100 * 100) : ((((neuron.dissolve_state?.WhenDissolvedTimestampSeconds || 1) - Math.round(new Date().getTime() / 1000)) / ((systemParams.max_dissolve_delay_seconds || 1) * (systemParams.max_dissolve_delay_bonus_percentage || 1))) * 100 * 100);
          const ageBonus = neuron.state?.split(/(?=[A-Z])/)
            .join(' ') === 'Dissolved' ? '-' : neuron.dissolve_state?.DissolveDelaySeconds ? ((neuron.current_age_seconds / systemParams.max_neuron_age_for_age_bonus || 1) * systemParams.max_age_bonus_percentage || 1) : 0;
          const totalBonus = (((1 + ageBonus / 100) * (1 + dissolveDelayBonus / 100)) - 1) * 100;
          return (
            <TableRow key={neuron.id}>
              <TableCell className='capitalize' aria-label={neuron.id}>
                <Accordion>
                  <AccordionItem
                    className=''
                    indicator={({ isOpen }) => (isOpen ? <SimpleDownArrow />
                      : <SimpleDownArrow className='rotate-90' />)}
                    title={
                      <div className='flex justify-between items-center text-start text-base font-normal h-14 gap-x-8'>
                        <h2 className='w-[150px] text-xs text-DarkGrey font-bold truncate' onClick={() => copyContent(neuron.id, setCopyState)}>{neuron.id}</h2>
                        <div className='w-[120px] h-[32px] px-4 rounded-full bg-DarkGrey flex items-center justify-center'>
                          <h2 className='text-white font-bold text-xs'>{neuron.state?.split(/(?=[A-Z])/)
                            .join(' ')}</h2>
                        </div>
                        <h2 className='w-[155px] ml-6'>
                          {(neuron.stake_e8s / 1e8) % 1 !== 0
                            ? (neuron.stake_e8s / 1e8).toFixed(2)
                            : neuron.stake_e8s / 1e8} GLDGov
                        </h2>
                        <h2 className='w-[80px] sm:mr-[1%]'>{neuron.total_maturity_e8s_equivalent / 1e8}</h2>
                        <h2 className='w-[120px] sm:mr-[1%]'>{elapsedTime(neuron.current_dissolve_delay_seconds)}</h2>
                        <h2 className='w-[75px]'>{elapsedTime(neuron.current_age_seconds)}</h2>
                        <h2 className='w-1/12 mr-6'>{neuron.voting_power === 0 ? '-' : parseNumbers((neuron.voting_power / 1e8).toFixed(0))}</h2>
                      </div>
                    }
                  >
                    <section
                      className="flex justify-start w-full gap-10 py-5"
                      style={{ transition: 'height 0.3s ease' }}
                    >
                      <div>
                        <p className="text-xs font-bold text-[#D3B871]">Date Created</p>
                        <p className="text-xs font-bold mt-2">
                          {formatDateFromSeconds(neuron.created_timestamp_seconds)}, {' '}
                          {elapsedTime(Math.round(new Date().getTime() / 1000)
                            - neuron.created_timestamp_seconds)}
                        </p>
                      </div>
                      <div>
                        <p className="text-xs font-bold text-[#D3B871]">Auto-Stake Maturity</p>
                        <p className="text-xs font-bold mt-2">
                          {neuron.auto_stake_maturity ? 'Yes' : 'No'}
                        </p>
                      </div>
                      <div>
                        <p className="text-xs font-bold text-[#D3B871]">Dissolve Delay Bonus</p>
                        <p className="text-xs font-bold mt-2">{dissolveDelayBonus?.toFixed?.(2) || '-'} %</p>
                      </div>
                      <div>
                        <p className="text-xs font-bold text-[#D3B871]">Age Bonus</p>
                        <p className="text-xs font-bold mt-2">{ageBonus?.toFixed?.(2) || '-'} %</p>
                      </div>
                      <div>
                        <p className="text-xs font-bold text-[#D3B871]">Total Bonus</p>
                        <p className="text-xs font-bold mt-2">{neuron.state?.split(/(?=[A-Z])/)
                          .join(' ') === 'Dissolved' ? '-' : (totalBonus).toFixed(2)} %</p>
                      </div>
                    </section>
                  </AccordionItem>
                </Accordion>
              </TableCell>
            </TableRow>
          );
        })}
        {!neurons.loading && (<TableRow className='sm:hidden'>
          <TableCell>
            <div className='flex gap-x-8 items-center'>
              <Pagination
                disableCursorAnimation
                showControls
                total={totalPages}
                page={page}
                onChange={(val) => setPage(val)}
                initialPage={1}
                className="gap-2"
                radius="full"
                renderItem={renderItem}
                variant="light"
              />
              <Select
                label="Rows per page"
                radius='full'
                variant="bordered"
                labelPlacement={'outside-left'}
                selectedKeys={[limit]}
                onChange={(e) => handleSelectChange(e)}
                className="w-48 items-center"
                selectorIcon={<ChevronDownArrow />}
                classNames={{
                  value: 'font-bold text-white',
                  popoverContent: '-mt-1 rounded-lg',
                  trigger: 'bg-[#D9D9D9] border-none min-h-fit h-8',
                  selectorIcon: 'text-black',
                }}
                listboxProps={{
                  itemClasses: {
                    title: 'font-bold',
                    base: [
                      'rounded-full',
                      'transition-opacity',
                      'data-[hover=true]:text-foreground',
                      'data-[hover=true]:bg-default-100',
                      'dark:data-[hover=true]:bg-default-50',
                      'data-[selectable=true]:focus:bg-default-50',
                      'data-[pressed=true]:opacity-70',
                      'data-[focus-visible=true]:ring-default-500',
                    ],
                  },
                }}
              >
                <SelectItem key={10} value={10}>
                  10
                </SelectItem>
                <SelectItem key={50} value={50}>
                  50
                </SelectItem>
                <SelectItem key={100} value={100}>
                  100
                </SelectItem>
              </Select>
            </div>
          </TableCell>
        </TableRow>)}
      </TableBody>
    </Table>
  );
};

export default NeuronsTable;
