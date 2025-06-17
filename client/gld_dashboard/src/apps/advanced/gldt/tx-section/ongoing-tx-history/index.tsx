import { useMemo, Fragment } from "react";
import {
  ColumnDef,
  useReactTable,
  getCoreRowModel,
  flexRender,
} from "@tanstack/react-table";
import { SWAP_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import useFetchNFTOnGoingTxs from "@shared/hooks/useFetchNFTUserOnGoingTxs";
import { SwapData } from "@services/gldt_swap/utils/interfaces";
import TxStatus from "@advanced/gldt/tx-section/shared/tx-history-table-cell/TxStatus";
import RenderValueNFT from "@advanced/gldt/tx-section/shared/tx-history-table-cell/ValueNFT";
import RenderValueGLDT from "@advanced/gldt/tx-section/shared/tx-history-table-cell/ValueGLDT";
import RenderValueCreatedAt from "@advanced/gldt/tx-section/shared/tx-history-table-cell/ValueCreatedAt";

const TxHistory = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();

  const ongoingTxs = useFetchNFTOnGoingTxs(
    SWAP_CANISTER_ID,
    unauthenticatedAgent,
    {
      principal: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
      refetchInterval: 3000,
    }
  );

  const data = useMemo<SwapData[]>(
    () => ongoingTxs.data ?? [],
    [ongoingTxs.data]
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

  return (
    <>
      <div className="overflow-x-auto w-full">
        {ongoingTxs.isLoading && (
          <div className="p-4 flex items-center justify-center">
            Loading on going transactions...
          </div>
        )}
        {ongoingTxs.isError && (
          <div className="flex justify-center items-center text-danger p-4">
            Fetching history error.
          </div>
        )}
        {ongoingTxs.isSuccess && data.length >= 1 && (
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
                          {flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
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
        {ongoingTxs.isSuccess && !data.length && (
          <div className="p-4 flex items-center justify-center">
            No ongoing transactions
          </div>
        )}
      </div>
    </>
  );
};

export default TxHistory;
