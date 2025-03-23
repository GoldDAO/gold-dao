import { useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { ColumnDef } from "@tanstack/react-table";

import { GOLDAO_LEDGER_CANISTER_ID_IC } from "@constants";

import { Table, LoaderSpin } from "@components/index";
import CopyToClipboard from "@components/buttons/CopyToClipboard";
import { BadgeTransactionKind } from "@components/badges/BadgeTransactionKind";

import { TableProps } from "@utils/table/useTable";

import useFetchAccountTransactions, {
  Transaction,
} from "@services/ledger/hooks/useFetchAccountTransactions";

export const GLDGovTransactionsAccountTable = ({
  pagination = {
    pageIndex: 0,
    pageSize: 10,
  },
  setPagination,
  sorting,
  setSorting,
  account,
}: TableProps & { account: string }) => {
  const navigate = useNavigate();

  const columns = useMemo<ColumnDef<Transaction>[]>(
    () => [
      {
        accessorKey: "index",
        id: "index",
        cell: ({ getValue }) => (
          <button
            onClick={() => handleNavigateTransaction(getValue() as string)}
            className="text-accent"
          >
            {getValue() as string}
          </button>
        ),
        header: "Index",
        meta: {
          className: "",
        },
      },
      {
        accessorKey: "amount",
        id: "amount",
        cell: ({ getValue }) => getValue(),
        header: "Amount",
      },
      {
        accessorKey: "kind",
        id: "kind",
        cell: ({ getValue }) => (
          <BadgeTransactionKind
            kind={getValue() as "mint" | "burn" | "approve" | "transfer"}
          />
        ),
        header: "Type",
      },
      {
        accessorKey: "timestamp",
        id: "timestamp",
        cell: ({ getValue }) => getValue() as string,
        header: "Date",
      },
      {
        accessorKey: "from_account",
        id: "from_account",
        cell: ({ getValue }) => (
          <div className="flex items-center max-w-64">
            {getValue() === "Minting account" && (
              <div>{getValue() as string}</div>
            )}
            {getValue() !== "Minting account" && (
              <>
                <button
                  onClick={() => handleNavigateAccount(getValue() as string)}
                  data-tooltip-id="tooltip"
                  data-tooltip-content={getValue() as string}
                  className="mr-2 truncate text-accent"
                >
                  {getValue() as string}
                </button>
                <CopyToClipboard value={getValue() as string} />
              </>
            )}
          </div>
        ),
        header: "From",
        enableSorting: false,
      },
      {
        accessorKey: "to_account",
        id: "to_account",
        cell: ({ getValue }) => (
          <div className="flex items-center max-w-64">
            {getValue() === "Minting account" && (
              <div>{getValue() as string}</div>
            )}
            {getValue() !== "Minting account" && (
              <>
                <button
                  onClick={() => handleNavigateAccount(getValue() as string)}
                  data-tooltip-id="tooltip"
                  data-tooltip-content={getValue() as string}
                  className="mr-2 truncate text-accent"
                >
                  {getValue() as string}
                </button>
                <CopyToClipboard value={getValue() as string} />
              </>
            )}
          </div>
        ),
        header: "To",
        enableSorting: false,
      },
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  const handleNavigateAccount = (account: string) => {
    navigate(`/dashboard/sns/accounts/${account}`);
  };

  const handleNavigateTransaction = (tx: string) => {
    navigate(`/dashboard/sns/transactions/${tx}`);
  };

  const { data, isSuccess, isLoading, isError } = useFetchAccountTransactions(
    GOLDAO_LEDGER_CANISTER_ID_IC,
    {
      limit: pagination.pageSize,
      offset: pagination.pageSize * pagination.pageIndex,
      sorting,
      account,
    }
  );

  return (
    <div>
      {isSuccess && (
        <Table
          columns={columns}
          data={data.data}
          rowCount={data.total_transactions}
          pagination={pagination}
          setPagination={setPagination}
          sorting={sorting}
          setSorting={setSorting}
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
