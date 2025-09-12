import { useMemo, useState } from "react";
import {
  useReactTable,
  getCoreRowModel,
  getPaginationRowModel,
  createColumnHelper,
  flexRender,
  getSortedRowModel,
  SortingState,
} from "@tanstack/react-table";
import { Tooltip } from "react-tooltip";
import { ClipboardDocumentIcon, CheckIcon } from "@heroicons/react/24/outline";
import useGetAllStakePositions, {
  Position,
} from "../../hooks/useGetAllStakePositions";
import { getCanister } from "../../utils/getCanister";
import NumberToLocaleString from "../shared/NumberToLocaleString";
import Card from "../shared/ui/Card";

const columnHelper = createColumnHelper<Position>();

const StakePositions = ({ env }: { env: string }) => {
  const [copiedPrincipal, setCopiedPrincipal] = useState<string | null>(null);
  const [sorting, setSorting] = useState<SortingState>([]);

  const stake_positions = useGetAllStakePositions(
    getCanister(env).GLDT_STAKE_CANISTER_ID
  );

  const handleCopyPrincipal = async (principal: string) => {
    try {
      await navigator.clipboard.writeText(principal);
      setCopiedPrincipal(principal);
      setTimeout(() => setCopiedPrincipal(null), 1500);
    } catch (error) {
      console.error("Failed to copy principal:", error);
    }
  };

  const columns = useMemo(
    () => [
      columnHelper.accessor("principal", {
        header: "Principal",
        cell: (info) => (
          <div className="flex items-center gap-2">
            <div
              className="font-mono text-sm"
              data-tooltip-id="principal-tooltip"
              data-tooltip-content={info.getValue()}
            >
              {info.getValue().slice(0, 20)}...
            </div>
            <button
              onClick={() => handleCopyPrincipal(info.getValue())}
              className="p-2 hover:bg-neutral-100 dark:hover:bg-neutral-800 rounded-full transition-colors cursor-pointer"
              title="Copy principal"
            >
              {copiedPrincipal === info.getValue() ? (
                <CheckIcon className="h-4 w-4 text-yellow-500" />
              ) : (
                <ClipboardDocumentIcon className="h-4 w-4 text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300" />
              )}
            </button>
          </div>
        ),
      }),
      columnHelper.accessor("staked_amount", {
        header: () => (
          <span className="cursor-pointer select-none">
            GLDT Staked
            <span className="ml-1">&#8597;</span>
          </span>
        ),
        cell: (info) => (
          <div className="text-center font-medium">
            <NumberToLocaleString value={info.getValue()} />
          </div>
        ),
        sortingFn: "basic",
      }),
      columnHelper.accessor("created_at", {
        header: () => (
          <span className="cursor-pointer select-none">
            Created At
            <span className="ml-1">&#8597;</span>
          </span>
        ),
        cell: (info) => {
          const date = new Date(Number(info.getValue()));
          return (
            <div className="text-center text-sm">
              {date.toLocaleDateString()}
            </div>
          );
        },
        sortingFn: "basic",
      }),
      columnHelper.group({
        header: "Unclaimed Rewards",
        columns: [
          columnHelper.accessor("rewards", {
            header: "GOLDAO",
            id: "goldao_rewards",
            cell: (info) => {
              const goldaoReward = info
                .getValue()
                .find((r) => r.name === "GOLDAO");
              return (
                <div className="text-center font-medium">
                  <NumberToLocaleString value={goldaoReward?.amount || 0} />
                </div>
              );
            },
          }),
          columnHelper.accessor("rewards", {
            header: "ICP",
            id: "icp_rewards",
            cell: (info) => {
              const icpReward = info.getValue().find((r) => r.name === "ICP");
              return (
                <div className="text-center font-medium">
                  <NumberToLocaleString value={icpReward?.amount || 0} />
                </div>
              );
            },
          }),
          columnHelper.accessor("rewards", {
            header: "OGY",
            id: "ogy_rewards",
            cell: (info) => {
              const ogyReward = info.getValue().find((r) => r.name === "OGY");
              return (
                <div className="text-center font-medium">
                  <NumberToLocaleString value={ogyReward?.amount || 0} />
                </div>
              );
            },
          }),
        ],
      }),
      columnHelper.accessor("dissolve_events", {
        header: "Dissolving Events",
        cell: (info) => {
          const events = info.getValue();
          return (
            <div className="text-sm">
              {events.map((event, index) => (
                <div key={index} className="mb-1">
                  <div className="font-medium">
                    <NumberToLocaleString value={event.amount} /> GLDT
                  </div>
                  <div className="text-xs text-neutral-500">
                    {event.is_withdrawable
                      ? "Withdrawable now"
                      : `${Math.ceil(
                          event.remaining_time / (1000 * 60 * 60 * 24)
                        )} days left`}
                  </div>
                </div>
              ))}
              {events.length === 0 && (
                <div className="text-neutral-400 text-xs">No events</div>
              )}
            </div>
          );
        },
      }),
    ],
    [copiedPrincipal]
  );

  const table = useReactTable({
    data: stake_positions.data ?? [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    initialState: {
      pagination: {
        pageSize: 10,
      },
    },
  });

  if (stake_positions.isLoading) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="">Fetching stake positions...</div>
      </div>
    );
  }

  return (
    <Card className="w-full">
      {/* Table */}
      <div className="overflow-x-auto">
        <table className="min-w-full">
          <thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr
                key={headerGroup.id}
                className="border-b border-neutral-200 dark:border-neutral-700"
              >
                {headerGroup.headers.map((header, index) => {
                  const isLeafColumn =
                    !header.subHeaders || header.subHeaders.length === 0;
                  return (
                    <th
                      key={header.id}
                      colSpan={header.colSpan}
                      className={`px-6 py-3 text-xs font-medium text-neutral-900/60 dark:text-neutral-50/60 uppercase tracking-wider ${
                        header.subHeaders?.length ||
                        header.id === "staked_amount" ||
                        header.id === "created_at" ||
                        header.id === "goldao_rewards" ||
                        header.id === "icp_rewards" ||
                        header.id === "ogy_rewards"
                          ? "text-center"
                          : "text-left"
                      }`}
                      {...(isLeafColumn && header.column.getCanSort()
                        ? {
                            onClick: header.column.getToggleSortingHandler(),
                            style: { cursor: "pointer" },
                          }
                        : {})}
                    >
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                      {isLeafColumn && header.column.getCanSort() && (
                        <span>
                          {header.column.getIsSorted() === "asc"
                            ? " ▲"
                            : header.column.getIsSorted() === "desc"
                            ? " ▼"
                            : ""}
                        </span>
                      )}
                    </th>
                  );
                })}
              </tr>
            ))}
          </thead>
          <tbody className="divide-y divide-neutral-200 dark:divide-neutral-700">
            {table.getRowModel().rows.map((row) => (
              <tr
                key={row.id}
                className="hover:bg-white dark:hover:bg-neutral-700/20"
              >
                {row.getVisibleCells().map((cell, index) => (
                  <td
                    key={cell.id}
                    className="px-6 py-4 whitespace-nowrap text-sm"
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      <div className="mt-8 px-6">
        <div className="flex flex-col space-y-3 sm:space-y-0 sm:flex-row sm:items-center sm:justify-between">
          {/* Results count */}
          <div className="flex justify-center sm:justify-start">
            <span className="text-sm text-neutral-900/60 dark:text-neutral-50/60">
              Showing{" "}
              {table.getState().pagination.pageIndex *
                table.getState().pagination.pageSize +
                1}{" "}
              to{" "}
              {Math.min(
                (table.getState().pagination.pageIndex + 1) *
                  table.getState().pagination.pageSize,
                table.getPrePaginationRowModel().rows.length
              )}{" "}
              of {table.getPrePaginationRowModel().rows.length} results
            </span>
          </div>

          {/* Page navigation */}
          <div className="flex items-center justify-center space-x-1 sm:space-x-2">
            <button
              onClick={() => table.setPageIndex(0)}
              disabled={!table.getCanPreviousPage()}
              className="px-2 sm:px-3 py-1 text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-white dark:hover:bg-neutral-700/20 rounded"
            >
              {"<<"}
            </button>

            <button
              onClick={() => table.previousPage()}
              disabled={!table.getCanPreviousPage()}
              className="px-2 sm:px-3 py-1 text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-white dark:hover:bg-neutral-700/20 rounded"
            >
              {"<"}
            </button>

            <span className="px-2 sm:px-3 py-1 text-sm whitespace-nowrap">
              Page {table.getState().pagination.pageIndex + 1} of{" "}
              {table.getPageCount()}
            </span>

            <button
              onClick={() => table.nextPage()}
              disabled={!table.getCanNextPage()}
              className="px-2 sm:px-3 py-1 text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-white dark:hover:bg-neutral-700/20 rounded"
            >
              {">"}
            </button>

            <button
              onClick={() => table.setPageIndex(table.getPageCount() - 1)}
              disabled={!table.getCanNextPage()}
              className="px-2 sm:px-3 py-1 text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-white dark:hover:bg-neutral-700/20 rounded"
            >
              {">>"}
            </button>
          </div>

          {/* Page size selector */}
          <div className="flex items-center justify-center space-x-2">
            <label className="text-sm text-neutral-900/60 dark:text-neutral-50/60 whitespace-nowrap">
              Show:
            </label>
            <select
              value={table.getState().pagination.pageSize}
              onChange={(e) => table.setPageSize(Number(e.target.value))}
              className="border border-neutral-200 dark:border-neutral-600 rounded px-2 py-1 text-sm bg-white dark:bg-neutral-800"
            >
              {[5, 10, 20, 50].map((pageSize) => (
                <option key={pageSize} value={pageSize}>
                  {pageSize}
                </option>
              ))}
            </select>
            <span className="text-sm text-neutral-900/60 dark:text-neutral-50/60 whitespace-nowrap">
              per page
            </span>
          </div>
        </div>
      </div>
      <Tooltip
        id="principal-tooltip"
        place="top"
        className="bg-white dark:bg-neutral-900 text-neutral-900 dark:text-white text-sm px-3 py-2 rounded-md shadow-sm"
      />
    </Card>
  );
};

export default StakePositions;
