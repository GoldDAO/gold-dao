'use client';

import React, { useEffect, useState } from 'react';
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
import useServices from '../../hooks/useServices';
import {
  ChevronDownArrow, ChevronRightArrow, InfoIcon, SimpleDownArrow,
} from '../../utils/svgs';
import { elapsedTime } from '../../utils/functions';
import usePagination from '../../hooks/usePagination';

const CanistersTable = () => {
  const { getCanisters } = useServices();
  const [canisters, setCanisters] = useState({ loading: true, data: [] });
  const {
    limit, setLimit, page, setPage, setItemsAmount, totalPages,
  } = usePagination();

  /*
    TODO:

      * hacer loader y mensaje de error
  */

  useEffect(() => {
    window.scrollTo(0, 0);
    (async () => {
      const data = await getCanisters();
      console.log({ data });
      setItemsAmount(data.length);
      setCanisters({ loading: false, data });
    })();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const tooltipClassName = {
    base: [
      'max-w-[400px]',
    ],
    content: [
      'py-2 px-4 shadow-xl',
      'text-white bg-black',
    ],
  };

  const renderItem = ({
    ref,
    key,
    value,
    isActive,
    activePage,
    onNext,
    onPrevious,
    setPage,
    className,
  }) => {
    if (value === PaginationItemType.NEXT) {
      return (
        <Button
          key={key}
          isIconOnly
          radius='full'
          size='sm'
          className='bg-transparent'
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
          radius='full'
          size='sm'
          className='bg-transparent rotate-180'
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
      return <Button key={key} className={className}>...</Button>;
    }

    // cursor is the default item
    return (
      <Button
        isIconOnly
        radius='full'
        size='sm'
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
        td: [
          'group-data-[middle=true]:before:rounded-none',
          'text-Taupe',
          'text-base',
        ],
      }}
    >
      <TableHeader>
        <TableColumn>
          <div className='flex justify-between mx-10'>
            <h1 className='text-xl font-bold text-start text-black mb-2'>Canisters</h1>
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
          <div className='flex justify-between text-start items-center mt-6 mx-6 h-12'>
            <h2 className="flex items-center gap-x-2 w-[181px]">
              ID
              <div className='hidden sm:block'>
                <Tooltip
                  content='The unique identifier of the canister.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center gap-x-2 w-[91px]">
              Type
              <div className="hidden sm:block">
                <Tooltip
                  content='The type of the canister.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center gap-x-2 w-[146px]">
              Cycles Balance
              <div className="hidden sm:block">
                <Tooltip
                  content='The number of GLDGov tokens staked in the canister. The dissolve delay bonus and age bonus are applied to this value plus staked maturity in order to calculate the voting power of the canister.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center gap-x-2 w-[234px]">
              Freezing Threshold Cycles
              <div className="hidden sm:block">
                <Tooltip
                  content='The minimum time period over which the canister owner locks up their staked GLDGov tokens. This determines how long it will take to dissolve if the canister is placed into the Dissolving state. Once a canister has been placed into the Dissolving state, its dissolve delay falls over the passage of time, rather like a kitchen timer, until either it is stopped or it reaches zero. When it reaches zero and enters the Dissolved state, its owner can perform a final disburse action to unlock the balance of GLDGov tokens. The dissolve delay can be configured up to a maximum of 2 years, and must be 91 days or greater for a canister to be able to vote and earn voting rewards.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center gap-x-2 w-[236px]">
              Idle Cycles Burned Per Day
              <div className="hidden sm:block">
                <Tooltip
                  content='The period of time that has elapsed since the canister was created or last entered the Not Dissolving state. While dissolving, a canister’s age is considered zero. Increasing the stake of a canister will decrease its age. For example, if the stake is doubled, the age will be halved. Splitting a canister creates a child canister that inherits the age of its parent.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center gap-x-2 w-[128px]">
              Memory Size
              <div className="hidden sm:block">
                <Tooltip
                  content='The voting power of the canister.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center gap-x-2 mr-2 ">
              Status
              <div className="hidden sm:block">
                <Tooltip
                  content='The voting power of the canister.'
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
        isLoading={canisters.loading}
        loadingContent={<Spinner className='mt-28' classNames={{ circle1: 'border-b-Beige', circle2: 'border-b-Beige' }} />}
      >
        {canisters.data.map((canister, index) => {
          const array = new Uint8Array(canister.ModuleHash);
          // eslint-disable-next-line no-bitwise
          const hexString = Array.from(array, (byte) => (`0${(byte & 0xff).toString(16)}`).slice(-2)).join('');
          return (
            <TableRow key={index}>
              <TableCell className='' aria-label={''}>
                <Accordion>
                  <AccordionItem
                    className=''
                    indicator={({ isOpen }) => (isOpen ? <SimpleDownArrow />
                      : <SimpleDownArrow className='rotate-90' />)}
                    title={
                      <div className='flex justify-between text-start items-center text-base font-normal h-14'>
                        <h2 className='text-xs text-DarkGrey font-bold underline w-[181px] mr-3 sm:mr-0'>{(canister.id)}</h2>
                        <h2 className='w-[91px] ml-3 sm:ml-0 text-center py-1 font-bold text-white rounded-full bg-[#C6C6C6]' >{canister.type}</h2>
                        <h2 className='w-[146px] ml-3 sm:ml-0'>
                          {canister.cycles}<span> T</span>
                        </h2>
                        <h2 className='w-[234px] ml-3 sm:ml-0'>{canister.freezingCycles}<span> T</span></h2>
                        <h2 className='w-[236px] ml-3 sm:ml-0'>{canister.idleCycles}</h2>
                        <h2 className='w-[128px] ml-3 sm:ml-0'>{canister.memory}<span> MiB</span></h2>
                        <h2 className=''>{canister.status}</h2>
                      </div>
                    }
                  >
                    <section
                      className="flex justify-start w-full gap-10 py-5"
                      style={{ transition: 'height 0.3s ease' }}
                    >
                      <div>
                        <p className="text-[12px] font-bold text-[#D3B871]">Freezing Threshold Time</p>
                        <p className="text-[12px] font-bold mt-2">
                          {elapsedTime(canister.freezingCycles) === 'a month'
                            ? '30 days'
                            : elapsedTime(canister.freezingCycles)}
                        </p>
                      </div>
                      <div>
                        <p className="text-[12px] font-bold text-[#D3B871]">Controllers</p>
                        <p className="text-[12px] font-bold mt-2">{canister.controllers}</p>
                      </div>
                      <div>
                        <p className="text-[12px] font-bold text-[#D3B871]">Module Hash</p>
                        <p className="text-[12px] font-bold mt-2">{hexString}</p>
                      </div>
                    </section>
                  </AccordionItem>
                </Accordion>
              </TableCell>
            </TableRow>
          );
        })}
        {!canisters.loading && <TableRow className='sm:hidden'>
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
        </TableRow>}
      </TableBody>
    </Table >
  );
};

export default CanistersTable;
