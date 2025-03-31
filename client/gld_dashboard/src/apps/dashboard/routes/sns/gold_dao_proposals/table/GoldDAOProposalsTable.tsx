import { useCallback, useMemo } from "react";
import { Link, useNavigate } from "react-router-dom";
import { ColumnDef, Row } from "@tanstack/react-table";
import { ChevronUpIcon, ChevronDownIcon } from "@heroicons/react/20/solid";

import { Table, LoaderSpin } from "@components/index";
import { BadgeProposalStatus } from "@components/badges/BadgeProposalStatus";
import { TableProps } from "@utils/table/useTable";

import useFetchAllProposals, {
  ProposalData,
} from "@services/sns_governance/hooks/useFetchAllProposals";
import { timestampToRelativeCalendar } from "@utils/dates";
import { numberToLocaleString } from "@utils/numbers";

const ProposalExpandedRow = ({
  row,
  navigate,
}: {
  row: Row<ProposalData>;
  navigate: ReturnType<typeof useNavigate>;
}) => {
  const proposal = row?.original;

  const handleOnClickProposer = () => {
    navigate(`/sns/neurons/${proposal.proposer}`);
  };

  const data = [
    {
      label: "Date Created",
      component: <div className="font-semibold">{proposal.created_at}</div>,
    },
    {
      label: "Proposer",
      component: (
        <button className="text-accent" onClick={handleOnClickProposer}>
          {proposal.proposer}
        </button>
      ),
    },
    {
      label: "URL",
      component: (
        <Link
          className="text-accent"
          to={proposal.url}
          target="_blank"
          rel="noopener noreferrer"
        >
          {proposal.url}
        </Link>
      ),
    },
    {
      label: "Yes",
      component: (
        <div className="flex gap-1 items-center">
          <div>{numberToLocaleString({ value: proposal.latestTally.yes })}</div>
          <div className="text-content/60">Votes</div>
        </div>
      ),
    },
    {
      label: "No",
      component: (
        <div className="flex gap-1 items-center">
          <div>{numberToLocaleString({ value: proposal.latestTally.no })}</div>
          <div className="text-content/60">Votes</div>
        </div>
      ),
    },
  ];
  if (!proposal) return null;

  return (
    <div className="flex border-b border-border">
      {data.map(({ label, component }) => (
        <div key={label} className="py-4 px-6 text-sm">
          <div className="text-content/60">{label}</div>
          {component}
        </div>
      ))}
    </div>
  );
};

export const GoldDAOProposalsTable = ({
  pagination = {
    pageIndex: 0,
    pageSize: 10,
  },
  setPagination,
  sorting,
  setSorting,
}: TableProps) => {
  const navigate = useNavigate();
  const handleNavigateProposalDetails = useCallback(
    (proposal: string) => {
      navigate(`/dashboard/sns/proposals/${proposal}`);
    },
    [navigate]
  );

  const columns = useMemo<ColumnDef<ProposalData>[]>(
    () => [
      {
        accessorKey: "id",
        id: "id",
        cell: ({ row, getValue }) =>
          row.getCanExpand() ? (
            <div className="flex items-center">
              <button
                {...{
                  onClick: row.getToggleExpandedHandler(),
                }}
                className="cursor-pointer mr-2"
              >
                {row.getIsExpanded() ? (
                  <ChevronUpIcon className="h-5 w-5" />
                ) : (
                  <ChevronDownIcon className="h-5 w-5" />
                )}
              </button>
              <div className="flex items-center max-w-sm">
                <button
                  className="mr-2 truncate text-accent"
                  onClick={() =>
                    handleNavigateProposalDetails(getValue() as string)
                  }
                >
                  {getValue() as string}
                </button>
              </div>
            </div>
          ) : (
            ""
          ),
        header: "ID",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "title",
        id: "title",
        cell: ({ getValue }) => (
          <div className="flex items-center max-w-64">
            <div
              data-tooltip-id="tooltip"
              data-tooltip-content={getValue() as string}
              className="mr-2 truncate"
            >
              {getValue() as string}
            </div>
          </div>
        ),
        header: "Title",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "type",
        id: "type",
        cell: ({ getValue }) => getValue(),
        header: "Type",
      },
      {
        accessorKey: "timeRemaining",
        id: "timeRemaining",
        cell: ({ getValue }) => (
          <div>{timestampToRelativeCalendar(getValue() as number)}</div>
        ),
        header: "Time Remaining",
      },
      {
        accessorKey: "status",
        id: "status",
        cell: ({ getValue }) => (
          <BadgeProposalStatus status={getValue() as "open" | "executed"} />
        ),
        header: "Status",
      },
      {
        accessorKey: "votes",
        id: "votes",
        cell: ({ getValue }) =>
          numberToLocaleString({
            value: getValue() as number,
            decimals: 0,
          }),
        header: "Votes",
      },
    ],
    [handleNavigateProposalDetails]
  );

  const { data, isSuccess, isLoading, isError } = useFetchAllProposals({
    limit: pagination.pageSize,
    offset: pagination.pageSize * pagination.pageIndex,
    sorting,
  });

  return (
    <div>
      {isSuccess && (
        <Table
          columns={columns}
          data={data.data}
          rowCount={data.total_proposals}
          pagination={pagination}
          setPagination={setPagination}
          sorting={sorting}
          setSorting={setSorting}
          getRowCanExpand={() => true}
          rowExpanded={({ row }) => (
            <ProposalExpandedRow row={row} navigate={navigate} />
          )}
        />
      )}
      {(isLoading || isError) && (
        <div className="flex justify-center py-8">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};
