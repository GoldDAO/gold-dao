/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { ColumnDef } from "@tanstack/react-table";
import { EyeIcon } from "@heroicons/react/24/outline";
// import CopyToClipboard from "@components/buttons/CopyToClipboard";
import { Table, LoaderSpin } from "@components/ui";
import TransactionStatus from "@components/transactions/badge/TransactionStatus";

import ColInfoGLDT from "../ColInfoGLDT";
import ColInfoNFT from "../ColInfoNFT";

import { TableProps } from "@utils/table/useTable";

import { SwapData } from "@canisters/gldt_swap/interfaces";
import { useGetUserActiveSwaps } from "@hooks/gldt_swap";
import ListEmptyItem from "./Empty";

const List = ({
  pagination,
  setPagination,
  sorting,
  setSorting,
}: TableProps) => {
  const navigate = useNavigate();

  const columns = useMemo<ColumnDef<SwapData>[]>(
    () => [
      {
        accessorKey: "index",
        id: "index",
        cell: ({ getValue }) => (
          <div className="font-semibold">{getValue()}</div>
        ),
        header: "Index ID",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        cell: ({ getValue }) => <div>{getValue()}</div>,
        header: "Created at",
        meta: {
          className: "text-left",
        },
      },
      {
        cell: ({ row, getValue }) => (
          <div className="flex justify-center">
            {row.original.type === "forward" && (
              <ColInfoNFT>{getValue()}</ColInfoNFT>
            )}
            {row.original.type === "reverse" && (
              <ColInfoGLDT>{getValue()}</ColInfoGLDT>
            )}
          </div>
        ),
        header: "Sending",
        accessorKey: "send_value",
        id: "send_value",
      },
      {
        cell: ({ row, getValue }) => (
          <div className="flex justify-center">
            {row.original.type === "forward" && (
              <ColInfoGLDT>{getValue()}</ColInfoGLDT>
            )}
            {row.original.type === "reverse" && (
              <ColInfoNFT>{getValue()}</ColInfoNFT>
            )}
          </div>
        ),
        header: "Receiving",
        accessorKey: "receive_value",
        id: "receive_value",
      },
      {
        accessorKey: "status",
        id: "status",
        cell: ({ getValue }) => (
          <div className="flex justify-center">
            <TransactionStatus status={getValue().label} />
          </div>
        ),
        header: "Status",
      },
      {
        header: "View",
        cell: (props) => (
          <div className="flex justify-center items-center shrink-0 rounded-full bg-surface border border-border hover:bg-surface-2 w-10 h-10">
            <button onClick={() => handleClickView(props)}>
              <EyeIcon className="h-5 w-5" />
            </button>
          </div>
        ),
        meta: {
          className: "text-center",
        },
      },
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  const active_swap = useGetUserActiveSwaps({
    refetchInterval: 10000,
  });

  const handleClickView = (cell) => {
    navigate(
      `/swap/account/transactions/${cell?.row?.original?.nft_id}?index=${cell?.row?.original?.index}`
    );
  };

  return (
    <div>
      {active_swap.isSuccess &&
        active_swap.data &&
        active_swap.data.rows.length !== 0 && (
          <Table
            columns={columns}
            data={active_swap.data}
            pagination={pagination}
            setPagination={setPagination}
            sorting={sorting}
            setSorting={setSorting}
          />
        )}
      {active_swap.isSuccess &&
        active_swap.data &&
        active_swap.data.rows.length === 0 && <ListEmptyItem />}
      {active_swap.isLoading && (
        <div className="flex items-center justify-center p-8">
          <LoaderSpin />
        </div>
      )}
      {active_swap.isError && (
        <div className="flex justify-center">
          <div className="p-8 border border-dark-orange bg-dark-orange/10 rounded-xl">
            <div className="text-dark-orange font-semibold">
              {active_swap.error}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default List;
