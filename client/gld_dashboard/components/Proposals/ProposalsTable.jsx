'use client';

import {
  Divider,
  Tooltip,
  Button,
  Pagination,
  PaginationItemType,
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
import useServices from '../../hooks/useServices';
import {
  CancelIcon, ChevronDownArrow, ChevronRightArrow, InfoIcon, OpenWindowIcon,
} from '../../utils/svgs';
import usePagination from '../../hooks/usePagination';

const ProposalsTable = () => {
  const { getProposals } = useServices();
  const [proposals, setProposals] = useState({ loading: true, data: [] });
  const {
    limit, setLimit, page, setPage, itemsAmount, setItemsAmount, totalPages,
  } = usePagination();

  useEffect(() => {
    window.scrollTo(0, 0);
    (async () => {
      setProposals({ ...proposals, loading: true });
      let data;
      if (page === 1) {
        data = await getProposals({ limit });
        // guardo el primer id de todas las proposals (cantidad de proposals)
        setItemsAmount(data[0]?.id);
      } else {
        // calculo el id de la proposal dependiendo el limit y la page
        const beforeProposal = Math.ceil(itemsAmount - limit * (page - 1) + 1);
        data = await getProposals({
          limit,
          before_proposal: [{ id: beforeProposal }],
        });
      }

      setProposals({ loading: false, data });
    })();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [page, limit]);

  const tooltipClassName = {
    base: ['max-w-[400px]'],
    content: ['py-2 px-4 shadow-xl', 'text-white bg-black'],
  };
  const openInNewTab = (url) => {
    window.open(url, '_blank', 'noreferrer');
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
            <h1 className='text-xl font-bold text-start text-black mb-2'>Proposals</h1>
            <div className='hidden sm:flex gap-x-8 items-center'>
              <Pagination
                disableCursorAnimation
                showControls
                total={totalPages}
                initialPage={1}
                page={page}
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
          <div className='flex justify-between items-center mt-6 mx-8 sm:mx-10 gap-8 sm:gap-0'>
            <h2 className="flex items-center justify-start gap-x-2 h-12 sm:w-1/12">
              ID
              <div className='hidden sm:block'>
                <Tooltip
                  content='The unique identifier of the proposal.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center justify-start gap-x-2 h-12 -ml-8 sm:-ml-1 sm:w-1/5">
              Title
              <div className='hidden sm:block'>
                <Tooltip
                  content='The title of the proposal.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center justify-start gap-x-2 h-12 pl-6 sm:pl-0 sm:ml-0 sm:w-1/5">
              Topic
              <div className='hidden sm:block'>
                <Tooltip
                  content='The topic of the proposal.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center justify-center gap-x-2 h-12 pr-5 sm:pr-0 sm:pl-1 sm:w-1/6 ">
              Status
              <div className='hidden sm:block'>
                <Tooltip
                  content='The status of the proposal.'
                  classNames={tooltipClassName}
                >
                  {InfoIcon}
                </Tooltip>
              </div>
            </h2>
            <h2 className="flex items-center justify-center gap-x-2 h-12 pr-5 sm:w-1/5 ">
              Votes
              <div className='hidden sm:block'>
                <Tooltip
                  content='The total number of yes and no votes for the proposal.'
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
        isLoading={proposals.loading}
        loadingContent={
          <Spinner
            className="mt-28"
            classNames={{ circle1: 'border-b-Beige', circle2: 'border-b-Beige' }}
          />
        }
      >
        {proposals.data.map((proposal) => {
          const percentageYes = proposal.votes
            ? ((proposal.votes.yes * 100) / proposal.votes.total).toFixed(3)
            : 0;
          const percentageNo = proposal.votes
            ? ((proposal.votes.no * 100) / proposal.votes.total).toFixed(3)
            : 0;
          return (
            <TableRow key={proposal.id}>
              <TableCell className='' aria-label={proposal.title}>
                <div className='flex justify-between gap-10 mx-6 items-center text-start text-base font-normal'>
                  <h2 className='w-1/12'>{proposal.id}</h2>
                  <h2 className='sm:w-1/5'>{proposal.title}</h2>
                  <h2 className='w-1/4 sm:w-1/5'>{proposal.topic}</h2>
                  <div className='w-1/6 flex justify-center'>
                    <div
                      className={`w-22 h-10 rounded-[4px] flex items-center justify-center text-white text-xs font-bold ${proposal.status.toLowerCase() === 'open' && 'bg-[#127E00]'} ${proposal.status.toLowerCase() === 'executed' && 'bg-DarkGrey'} ${proposal.status.toLowerCase() === 'failed' && 'bg-[#DE3D3D]'}`}
                    >
                      {proposal.status}
                    </div>
                  </div>
                  <div className="w-1/5 flex justify-center gap-x-2 ">
                    {proposal.status.toLowerCase() === 'executed' && (
                      <>
                        <h2>Yes: {percentageYes} %</h2>
                        <Divider orientation='vertical' className='h-17 sm:h-6 bg-black' />
                        <h2>No: {percentageNo} %</h2>
                      </>
                    )}
                    {proposal.status.toLowerCase() === 'open' && (
                      <Button
                        radius="full"
                        className="bg-black text-white text-xs font-bold z-10"
                        onClick={() => openInNewTab(`https://nns.ic0.app/proposal/?u=tw2vt-hqaaa-aaaaq-aab6a-cai&proposal=${proposal.id}`)
                        }
                      >
                        Vote now {OpenWindowIcon}
                      </Button>
                    )}
                    {proposal.status.toLowerCase().includes('fail') && <CancelIcon />}
                  </div>
                </div>
              </TableCell>
            </TableRow>
          );
        })}
        {!proposals.loading && (
          <TableRow className='sm:hidden'>
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
          </TableRow>
        )}
      </TableBody>
    </Table>
  );
};

export default ProposalsTable;
