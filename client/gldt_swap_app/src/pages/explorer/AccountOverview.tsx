import { useMemo } from "react";
import { useSearchParams, useNavigate } from "react-router-dom";
import { CellContext, ColumnDef } from "@tanstack/react-table";
import { BugAntIcon } from "@heroicons/react/24/solid";

import { Table, LoaderSpin } from "@components/ui";
import { BadgeTransactionType } from "@components/shared/badge/TransactionType";
import { usePagination } from "@utils/table/useTable";
import CopyToClipboard from "@components/shared/button/CopyToClipboard";

import NavbarHome from "@components/shared/navbars/Home";
import { AccountBalanceGLDT } from "@components/explorer/card/AccountBalanceGLDT";
import { FullAccount } from "@components/explorer/card/FullAccount";
import { OwnerSubaccounts } from "@components/explorer/card/OwnerSubaccounts";

import { Transaction, TxAccount } from "@hooks/gldt_ledger_indexer/utils";
import { useFetchLedgerAccountTransactions } from "@hooks/gldt_ledger_indexer/useFetchLedgerAccountTransactions";
import { Breadcrumb } from "@components/explorer/Breadcrumb";

export const AccountOverview = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const [pagination, setPagination] = usePagination();
  // const [sorting, setSorting] = useSorting({});
  const owner = searchParams.get("owner") as string;
  const subaccount = searchParams.get("subaccount") as string | undefined;

  const { data, isSuccess, isLoading, isError, error } =
    useFetchLedgerAccountTransactions({
      pageSize: pagination.pageSize,
      start: undefined,
      owner,
      subaccount,
    });

  const handleClickCol = (cell: CellContext<Transaction, unknown>) => {
    const columnId = cell.column.id;
    const row = cell.row.original;
    const pathnames: { [key: string]: string } = {
      index: `/explorer/transaction/${row.index}`,
      to: `/explorer/account?owner=${row.to?.owner}${
        row.to?.subaccount ? `&subaccount=${row.to?.subaccount}` : ""
      }&page_size=${pagination.pageSize}&page_index=0`,
      from: `/explorer/account?owner=${row.from?.owner}${
        row.from?.subaccount ? `&subaccount=${row.from?.subaccount}` : ""
      }&page_size=${pagination.pageSize}&page_index=0`,
    };
    navigate(pathnames[columnId]);
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
      // {
      //   accessorKey: "hash",
      //   id: "hash",
      //   cell: (info) => {
      //     const value = info.getValue() as string;
      //     return value ? (
      //       <div className="flex items-center max-w-32">
      //         <button
      //           onClick={() => handleClickCol(info)}
      //           data-tooltip-id="tooltip"
      //           data-tooltip-content={value}
      //           className="mr-2 truncate"
      //         >
      //           {value}
      //         </button>
      //         <CopyToClipboard value={value} />
      //       </div>
      //     ) : null;
      //   },
      //   header: "Hash",
      //   meta: {
      //     className: "",
      //   },
      // },
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
          // <Badge className="bg-gold/20 px-2">
          //   <div className="text-gold text-xs font-semibold shrink-0">
          //     {getValue() as string}
          //   </div>
          // </Badge>
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
    [pagination]
  );
  return (
    <>
      <div className="bg-surface-2">
        <NavbarHome />
        <section className="container mx-auto px-4 py-8 xl:py-16">
          <Breadcrumb owner={owner} subaccount={subaccount} />
          <div className="my-8">
            <div className="text-4xl font-semibold text-gold">GLDT</div>
            <div className="text-4xl">Account Overview</div>
          </div>
          <div className="mt-16">
            <div className="grid grid-cols-1 lg:grid-cols-3 lg:items-center gap-4 mb-8 h-42">
              <FullAccount
                owner={owner}
                subaccount={subaccount}
                className="h-full"
              />
              <OwnerSubaccounts
                owner={owner}
                subaccount={subaccount}
                className="h-full"
              />
              <AccountBalanceGLDT
                owner={owner}
                subaccount={subaccount}
                className="h-full"
              />
            </div>

            {isLoading && (
              <div className="flex justify-center my-16">
                <LoaderSpin />
              </div>
            )}
            {isSuccess &&
              (data.hasResults ? (
                <Table
                  columns={columns}
                  data={data}
                  pagination={pagination}
                  setPagination={setPagination}
                  // sorting={sorting}
                  // setSorting={setSorting}
                  serverSide={false}
                />
              ) : (
                <div className="text-center my-16">
                  No transactions found for this subaccount.
                </div>
              ))}
            {isError && (
              <div className="flex flex-col justify-center items-center my-16">
                <div>
                  <BugAntIcon className="size-16 mb-6 text-gold/80 animate-bounce" />
                </div>
                <div>{error.message}</div>
              </div>
            )}
          </div>
        </section>
      </div>
    </>
  );
};
