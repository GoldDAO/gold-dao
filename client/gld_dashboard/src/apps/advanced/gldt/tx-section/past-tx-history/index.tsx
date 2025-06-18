import { useMemo, useEffect, Fragment } from "react";
import {
  ColumnDef,
  useReactTable,
  getCoreRowModel,
  flexRender,
} from "@tanstack/react-table";
import { useInView } from "react-intersection-observer";
import { SWAP_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import useFetchNFTUserHistoryTxs from "@shared/hooks/useFetchNFTUserHistoryTxs";
import { SwapData } from "@services/gldt_swap/utils/interfaces";
import TxStatus from "@advanced/gldt/tx-section/shared/tx-history-table-cell/TxStatus";
import RenderValueNFT from "@advanced/gldt/tx-section/shared/tx-history-table-cell/ValueNFT";
import RenderValueGLDT from "@advanced/gldt/tx-section/shared/tx-history-table-cell/ValueGLDT";
import RenderValueCreatedAt from "@advanced/gldt/tx-section/shared/tx-history-table-cell/ValueCreatedAt";

const TxHistory = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();

  const history = useFetchNFTUserHistoryTxs(
    SWAP_CANISTER_ID,
    unauthenticatedAgent,
    {
      principal: principalId,
      limit: 10,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const data = useMemo<SwapData[]>(
    () => (history.data ? history.data.pages.flatMap((page) => page.data) : []),
    [history.data]
  );

  const columns = useMemo<ColumnDef<SwapData>[]>(
    () => [
      {
        accessorKey: "index",
        id: "index",
        cell: ({ getValue }) => getValue() as string,
        header: "Index ID",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        cell: ({ getValue }) => (
          <RenderValueCreatedAt value={getValue() as string} />
        ),
        header: "Created at",
        meta: {
          className: "text-left",
        },
      },
      {
        cell: ({ row, getValue }) =>
          row.original.type === "forward" ? (
            <RenderValueNFT value={getValue() as number} />
          ) : (
            <RenderValueGLDT value={getValue() as number} />
          ),
        header: "Sending",
        accessorKey: "send_value",
        id: "send_value",
        meta: {
          className: "text-left",
        },
      },
      {
        cell: ({ row, getValue }) =>
          row.original.type === "forward" ? (
            <RenderValueGLDT value={getValue() as number} />
          ) : (
            <RenderValueNFT value={getValue() as number} />
          ),
        header: "Receiving",
        accessorKey: "receive_value",
        id: "receive_value",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "status",
        id: "status",
        cell: ({ getValue }) => (
          <div className="flex justify-center">
            <TxStatus status={(getValue() as SwapData["status"]).label} />
          </div>
        ),
        header: "Status",
      },
      //   {
      //     header: "View",
      //     cell: () => (
      //       <div className="flex justify-center items-center shrink-0 rounded-full bg-surface border border-border hover:bg-surface-2 w-10 h-10">
      //         <button onClick={() => null}>
      //           <Eye className="h-5 w-5" />
      //         </button>
      //       </div>
      //     ),
      //     meta: {
      //       className: "text-center",
      //     },
      //   },
    ],
    []
  );

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  const { ref, inView } = useInView({
    threshold: 0,
    triggerOnce: false,
  });

  useEffect(() => {
    if (inView && history.hasNextPage && !history.isFetchingNextPage) {
      history.fetchNextPage();
    }
  }, [inView, history]);

  return (
    <>
      <div className="overflow-x-auto w-full">
        {history.isLoading && (
          <div className="flex justify-center items-center p-4">
            <div>Loading past transaction...</div>
          </div>
        )}
        {history.isError && (
          <div className="flex justify-center items-center p-4 text-danger">
            <div>Fetching history error.</div>
          </div>
        )}
        {history.isSuccess && (
          <table className="table-auto w-full">
            <thead className="text-content/60">
              {table.getHeaderGroups().map((headerGroup) => (
                <tr key={headerGroup.id}>
                  {headerGroup.headers.map((header) => (
                    <th
                      key={header.id}
                      colSpan={header.colSpan}
                      className="py-4 px-8 font-normal text-sm"
                    >
                      {header.isPlaceholder ? null : (
                        <div
                          className={`flex items-center ${
                            (
                              header.column.columnDef.meta as {
                                className?: string;
                              }
                            )?.className ?? "justify-center"
                          }`}
                        >
                          <div className="flex shrink-0">
                            {flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                          </div>
                        </div>
                      )}
                    </th>
                  ))}
                </tr>
              ))}
            </thead>
            <tbody>
              {table.getRowModel().rows.map((row) => (
                <Fragment key={row.id}>
                  <tr className="border-b last:border-none border-border text-sm">
                    {row.getVisibleCells().map((cell) => (
                      <td
                        key={cell.id}
                        className={`px-8 py-4 overflow-hidden text-ellipsis whitespace-nowrap ${
                          (
                            cell.column.columnDef.meta as {
                              className?: string;
                            }
                          )?.className ?? "text-center"
                        }`}
                      >
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </td>
                    ))}
                  </tr>
                </Fragment>
              ))}
            </tbody>
          </table>
        )}
        <div ref={ref} />
      </div>
      {history.isFetchingNextPage && (
        <div className="p-4 flex items-center justify-center">
          Loading past transactions...
        </div>
      )}
      {!history.hasNextPage &&
        !history.isLoading &&
        !history.isError &&
        data.length >= 1 && (
          <div className="p-4 flex items-center justify-center">
            No more results
          </div>
        )}
      {history.isSuccess && !data.length && (
        <div className="p-4 flex items-center justify-center">
          No past transactions
        </div>
      )}
    </>
  );
};

export default TxHistory;
