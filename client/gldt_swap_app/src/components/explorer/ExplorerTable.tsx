import { useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { CellContext, ColumnDef } from "@tanstack/react-table";
import { BugAntIcon } from "@heroicons/react/24/solid";

import { Table, LoaderSpin } from "@components/ui";
import { BadgeTransactionType } from "@components/shared/badge/TransactionType";
import { usePagination } from "@utils/table/useTable";
import CopyToClipboard from "@components/shared/button/CopyToClipboard";

import {
  useFetchLedgerTransactions,
  Transaction,
  TxAccount,
} from "@hooks/gldt_ledger_indexer/useFetchLedgerTransactions";

export const ExplorerTable = () => {
  const navigate = useNavigate();
  const [pagination, setPagination] = usePagination();
  const { data, isSuccess, isLoading, isError, error } =
    useFetchLedgerTransactions({
      pageSize: pagination.pageSize,
      page: pagination.pageIndex,
    });

  const handleClickCol = (cell: CellContext<Transaction, unknown>) => {
    const columnId = cell.column.id;
    const row = cell.row.original;
    if (columnId === "index") navigate(`/explorer/transaction/${row.index}`);
    else {
      const account = columnId === "to" ? row.to : row.from;
      navigate(
        `/explorer/transactions/account?owner=${account?.owner}${
          account?.subaccount ? `&subaccount=${account?.subaccount}` : ""
        }`
      );
    }
  };

  const columns = useMemo<ColumnDef<Transaction>[]>(
    () => [
      {
        accessorKey: "index",
        id: "index",
        cell: (info) => {
          const value = info.getValue() as string;
          return (
            <div className="flex items-center">
              <button onClick={() => handleClickCol(info)}>{value}</button>
            </div>
          );
        },
        header: "Index",
        meta: {
          className: "",
        },
      },
      {
        accessorKey: "hash",
        id: "hash",
        cell: (info) => {
          const value = info.getValue() as string;
          return value ? (
            <div className="flex items-center max-w-32">
              <div
                data-tooltip-id="tooltip"
                data-tooltip-content={value}
                className="mr-2 truncate"
              >
                {value}
              </div>
              <CopyToClipboard value={value} />
            </div>
          ) : null;
        },
        header: "Hash",
        meta: {
          className: "",
        },
      },
      {
        accessorKey: "date",
        id: "date",
        cell: ({ getValue }) => {
          return <div className="text-sm">{getValue() as string}</div>;
        },
        header: "Date",
      },
      {
        accessorKey: "type",
        id: "type",
        cell: ({ getValue }) => (
          <BadgeTransactionType type={getValue() as string} />
        ),
        header: "Type",
      },
      {
        accessorKey: "amount",
        id: "amount",
        cell: ({ getValue }) => getValue(),
        header: "Amount",
      },
      {
        accessorKey: "from",
        id: "from",
        cell: (info) => {
          const value = info.getValue() as TxAccount;
          if (!value) return <div>-</div>;
          return (
            <div className="flex items-center max-w-56">
              {value.full && value.full === "Minting account" ? (
                <div>{value.full}</div>
              ) : (
                <>
                  <button
                    onClick={() => handleClickCol(info)}
                    data-tooltip-id="tooltip"
                    data-tooltip-content={value.full}
                    className="mr-2 truncate"
                  >
                    {value.full}
                  </button>
                  <CopyToClipboard value={value.full} />
                </>
              )}
            </div>
          );
        },
        header: "From",
      },
      {
        accessorKey: "to",
        id: "to",
        cell: (info) => {
          const value = info.getValue() as TxAccount;
          if (!value) return <div>-</div>;
          return (
            <div className="flex items-center max-w-56">
              {value.full && value.full === "Minting account" ? (
                <div>{value.full}</div>
              ) : (
                <>
                  <button
                    onClick={() => handleClickCol(info)}
                    data-tooltip-id="tooltip"
                    data-tooltip-content={value.full}
                    className="mr-2 truncate"
                  >
                    {value.full}
                  </button>
                  <CopyToClipboard value={value.full} />
                </>
              )}
            </div>
          );
        },
        header: "To",
      },
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  return (
    <>
      {isLoading && (
        <div className="flex justify-center">
          <LoaderSpin />
        </div>
      )}
      {isSuccess && (
        <Table
          columns={columns}
          data={data}
          pagination={pagination}
          setPagination={setPagination}
        />
      )}
      {isError && (
        <div className="flex flex-col justify-center items-center">
          <div>
            <BugAntIcon className="size-16 mb-6 text-gold/80 animate-bounce" />
          </div>
          <div>{error.message}</div>
        </div>
      )}
    </>
  );
};
