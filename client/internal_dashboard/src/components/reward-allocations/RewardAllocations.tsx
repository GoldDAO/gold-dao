import { useMemo } from "react";
import {
  useReactTable,
  getCoreRowModel,
  getPaginationRowModel,
  createColumnHelper,
  flexRender,
} from "@tanstack/react-table";
import { Tooltip } from "react-tooltip";
import useGetRewardAllocations, {
  DailyAnalytic,
} from "../../hooks/useGetRewardAllocations";
import { getCanister } from "../../utils/getCanister";
import NumberToLocaleString from "../shared/NumberToLocaleString";
import Card from "../shared/ui/Card";

const columnHelper = createColumnHelper<DailyAnalytic>();

const RewardAllocations = ({ env }: { env: string }) => {
  const reward_allocations = useGetRewardAllocations(
    getCanister(env).GLDT_STAKE_CANISTER_ID
  );

  const columns = useMemo(
    () => [
      columnHelper.accessor("date", {
        header: "Date",
        cell: (info) => {
          const date = new Date(Number(info.getValue()));
          return <div className="text-sm">{date.toLocaleDateString()}</div>;
        },
      }),
      columnHelper.accessor("staked_gldt", {
        header: "Total GLDT Staked",
        cell: (info) => (
          <div className="text-center font-medium">
            <NumberToLocaleString value={info.getValue()} />
          </div>
        ),
      }),
      columnHelper.group({
        header: "Allocated Rewards",
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
    ],
    []
  );

  const table = useReactTable({
    data: reward_allocations.data ?? [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    initialState: {
      pagination: {
        pageSize: 10,
      },
    },
  });

  if (reward_allocations.isLoading) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="">Fetching reward allocations...</div>
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
                {headerGroup.headers.map((header, index) => (
                  <th
                    key={header.id}
                    colSpan={header.colSpan}
                    className={`px-6 py-3 text-xs font-medium text-neutral-900/60 dark:text-neutral-50/60 uppercase tracking-wider ${
                      header.subHeaders?.length ||
                      header.id === "staked_gldt" ||
                      header.id === "goldao_rewards" ||
                      header.id === "icp_rewards" ||
                      header.id === "ogy_rewards"
                        ? "text-center"
                        : "text-left"
                    }`}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </th>
                ))}
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

export default RewardAllocations;
