/* eslint-disable no-console */

'use client';

import {
  Divider,
  Tooltip,
  Button,
  Pagination,
  PaginationItemType,
  SelectItem,
  Select,
  Spinner,
  Table,
  TableBody,
  TableRow,
  TableCell,
  TableHeader,
  TableColumn,
} from '@nextui-org/react';
import { useEffect, useState } from 'react';
import {
  ChevronDownArrow, ChevronRightArrow, InfoIcon, TransferIcon,
} from '../../utils/svgs';
import usePagination from '../../hooks/usePagination';
import useActor from '../../hooks/useActor';
import { elapsedTime, formatDateFromSeconds } from '../../utils/functions';

const URL = 'https://icrc-api.internetcomputer.org/api/v1/ledgers/tyyy3-4aaaa-aaaaq-aab7a-cai/transactions';

const TransactionsTable = () => {
  const [ledger] = useActor('ledger');
  const [transactions, setTransactions] = useState({ loading: true, data: [] });
  const {
    limit, setLimit, page, setPage, setItemsAmount, totalPages,
  } = usePagination();

  useEffect(() => {
    window.scrollTo(0, 0);
    (async () => {
      try {
        // obtenemos el maximo index de las txs (cantidad de txs)
        const resLedger = await ledger.get_transactions({
          start: 0,
          length: 0,
        });

        const maxIndex = Number(resLedger?.log_length || 102490n);
        const res = await fetch(`${URL}?max_transaction_index=${maxIndex}&offset=${(page - 1) * limit}&limit=${limit}`);
        const data = await res.json();

        setItemsAmount(maxIndex);
        setTransactions({ loading: false, data: data?.data });
      } catch (err) {
        console.log('get txs fetch error:', err);
        setTransactions({ loading: false, data: [] });
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
    setPage: setPageFn,
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
        onClick={() => setPageFn(value)}
      >
        {value}
      </Button>
    );
  };

  const handleSelectChange = (e) => {
    setLimit(parseInt(e.target.value, 10));
    setPage(1);
  };

  const openInNewTab = (url) => {
    window.open(url, '_blank', 'noreferrer');
  };

  return (
    <Table
      shadow="none"
      isHeaderSticky={true}
      hideScrollbar={true}
      radius="none"
      className="max-w-full sm:max-w-[1400px] sm:mx-auto sm:mt-10 sm:mb-44 overflow-hidden scrollbar-thin"
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
            <h1 className='text-xl font-bold text-start text-black mb-2'>Transactions</h1>
            <div className='gap-x-8 items-center hidden sm:flex'>
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
          <div className='flex justify-between items-center mt-6 mx-8 sm:mx-10 gap-16 sm:gap-0'>
            <h2 className="flex items-center justify-start gap-x-2 h-12 sm:w-1/12 ">
              Index
              <Tooltip
                content="The index of the transaction in the GLDGov ledger."
                classNames={tooltipClassName}
              >
                {InfoIcon}
              </Tooltip>
            </h2>
            <h2 className="flex items-center justify-start gap-x-2 h-12 sm:w-1/12 ">
              Amount
              <Tooltip
                content="The amount of GLDGov tokens transferred, or for 'approve' transactions, the designated amount of GLDGov tokens that the 'Spender Account' is authorized to transfer on behalf of the 'From' account."
                classNames={tooltipClassName}
              >
                {InfoIcon}
              </Tooltip>
            </h2>
            <h2 className="flex items-center justify-start gap-x-2 h-12 pl-3 sm:pl-0 sm:w-1/12 ">
              Type
              <Tooltip
                content="The type of GLDGov transaction (i.e., mint, burn, approve, or transfer)."
                classNames={tooltipClassName}
              >
                {InfoIcon}
              </Tooltip>
            </h2>
            <h2 className="flex items-center justify-start gap-x-2 h-12 -ml-4 sm:-ml-0 sm:w-1/6 ">
              Timestamp
              <Tooltip content="The date the GLDGov ledger constructed the block containing the transaction." classNames={tooltipClassName}>
                {InfoIcon}
              </Tooltip>
            </h2>
            <h2 className="flex items-center justify-start -ml-7 sm:-ml-0 gap-x-2 h-12 w-1/6">
              From
              <Tooltip
                content="The account that GLDGov tokens were transferred from, or for 'approve' transactions, the account whose owner has authorized the 'spender' account to transfer a designated amount of GLDGov tokens from the account on their behalf."
                classNames={tooltipClassName}
              >
                {InfoIcon}
              </Tooltip>
            </h2>
            <h2 className="flex items-center justify-start -ml-1 sm:-ml-0 gap-x-2 h-12 w-1/6">
              To
              <Tooltip
                content="The account that GLDGov tokens were transferred to. Not applicable for 'approve' transactions."
                classNames={tooltipClassName}
              >
                {InfoIcon}
              </Tooltip>
            </h2>
          </div>
        </TableColumn>
      </TableHeader>
      <TableBody
        isLoading={transactions.loading}
        loadingContent={
          <Spinner
            className="mt-28"
            classNames={{ circle1: 'border-b-Beige', circle2: 'border-b-Beige' }}
          />
        }
      >
        {transactions.data.map((tx) => (
          <TableRow key={tx.index}>
            <TableCell className='' aria-label={tx.index}>
              <div className='flex justify-between gap-16 mx-6 items-center text-start text-base font-normal'>
                <h2 className='w-1/12'>{tx.index}</h2>
                <h2 className='sm:w-1/12'>{(tx.amount / 1e8) % 1 !== 0 ? (tx.amount / 1e8).toFixed(2) : tx.amount / 1e8} GLDGov</h2>
                <h2 className=' py-2 px-4 text-white text-center font-bold rounded-full bg-[#C6C6C6] flex justify-center items-center gap-2'>{tx.kind.charAt(0).toUpperCase() + tx.kind.slice(1)} {TransferIcon}</h2>

                <h2 className='w-1/6'>{formatDateFromSeconds((tx.timestamp / 1000000000))},{' '}
                        {elapsedTime(Math.round(new Date().getTime() / 1000)
                          - (tx.timestamp / 1000000000))}</h2>
                <h2 className='w-1/6  underline'
                onClick={() => tx.kind === 'transfer' && openInNewTab(`https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/account/${tx.from_owner}`)}>{tx.kind === 'transfer' ? tx.from_owner : 'Minting Account'}</h2>
                <h2 className='w-1/6  underline'
                onClick={() => openInNewTab(`https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/account/${tx.to_owner}`)}>{tx.to_owner}</h2>
              </div>
            </TableCell>
          </TableRow>
        ))}
        {!transactions.loading && (
          <TableRow>
            <TableCell>
              <div className='flex gap-x-8 items-center sm:hidden'>
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
          </TableRow>
        )}
      </TableBody>
    </Table>
  );
};

export default TransactionsTable;
