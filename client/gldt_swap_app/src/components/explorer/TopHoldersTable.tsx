import { useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { CellContext, ColumnDef } from "@tanstack/react-table";
import { BugAntIcon } from "@heroicons/react/24/solid";

import { Table, LoaderSpin } from "@components/ui";
import CopyToClipboard from "@components/shared/button/CopyToClipboard";

import {
  useFetchTopAccountHolders,
  AccountTopHolder,
  TxAccount,
} from "@hooks/gldt_super_stats_v3/index";

export const TopHoldersTable = () => {
  const navigate = useNavigate();
  const { data, isSuccess, isLoading, isError, error } =
    useFetchTopAccountHolders();

  const handleClickCol = (cell: CellContext<AccountTopHolder, unknown>) => {
    const row = cell.row.original;
    navigate(
      `/explorer/top_holders/account?owner=${row.address?.owner}${
        row.address?.subaccount ? `&subaccount=${row.address?.subaccount}` : ""
      }`
    );
  };

  const columns = useMemo<ColumnDef<AccountTopHolder>[]>(
    () => [
      {
        accessorKey: "rank",
        id: "rank",
        cell: ({ getValue }) => getValue(),
        header: "Rank",
        meta: {
          className: "",
        },
      },
      {
        accessorKey: "address",
        id: "address",
        cell: (info) => {
          const value = info.getValue() as TxAccount;
          if (!value) return <div>-</div>;
          return (
            <div className="flex items-center">
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
        header: "Address",
        meta: {
          className: "",
        },
      },
      {
        accessorKey: "quantity",
        id: "quantity",
        cell: ({ getValue }) => getValue(),
        header: "Quantity",
      },
      {
        accessorKey: "percentage",
        id: "percentage",
        cell: ({ getValue }) => <div>{getValue() as string} %</div>,
        header: "Percentage",
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
      {isSuccess && <Table columns={columns} data={data} />}
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
