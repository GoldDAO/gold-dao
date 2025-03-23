import { useMemo } from "react";
// import { useNavigate } from "react-router-dom";
import { ColumnDef } from "@tanstack/react-table";
import { Table, LoaderSpin } from "@components/index";

// import CopyToClipboard from "@components/buttons/CopyToClipboard";
import {
  BadgeCanisterType,
  CanisterType,
} from "@components/badges/BadgeCanisterType";
import {
  BadgeCanisterStatus,
  CanisterStatus,
} from "@components/badges/BadgeCanisterStatus";
import { TableProps } from "@utils/table/useTable";

import useFetchAllCanisters, {
  SNSCanistersSummaryData,
} from "@services/sns_root/hooks/useFetchAllCanisters";

import { useAuth } from "@auth/index";
import { SNS_ROOT_CANISTER_ID } from "@constants";

export const GoldDAOCanistersTable = ({
  pagination = {
    pageIndex: 0,
    pageSize: 10,
  },
  setPagination,
  sorting,
  setSorting,
}: TableProps) => {
  // const navigate = useNavigate();
  const { unauthenticatedAgent } = useAuth();

  const columns = useMemo<ColumnDef<SNSCanistersSummaryData>[]>(
    () => [
      {
        accessorKey: "canisterID",
        id: "canisterID",
        cell: (info) => info.getValue(),
        //   <button onClick={() => handleClickView(info)}>
        //     {info.getValue()}
        //   </button>
        header: "ID",
        meta: {
          className: "",
        },
      },
      {
        accessorKey: "type",
        id: "type",
        cell: ({ getValue }) => (
          <BadgeCanisterType type={getValue() as CanisterType} />
        ),
        header: "Type",
      },
      {
        accessorKey: "cyclesBalance",
        id: "cyclesBalance",
        cell: ({ getValue }) => <div>{getValue() as string} T</div>,
        header: "Cycles Balance",
      },
      {
        accessorKey: "freezingThresholdCycles",
        id: "freezingThresholdCycles",
        cell: ({ getValue }) => <div>{getValue() as string} T</div>,
        header: "Freezing Threshold Cycles",
      },
      {
        accessorKey: "idleCyclesBurnedPerDay",
        id: "idleCyclesBurnedPerDay",
        cell: (info) => info.getValue(),
        header: "Idle Cycles Burned Per Day",
      },
      {
        accessorKey: "memorySize",
        id: "memorySize",
        cell: ({ getValue }) => <div>{getValue() as string} MiB</div>,
        header: "Memory Size",
      },
      {
        accessorKey: "status",
        id: "status",
        cell: ({ getValue }) => (
          <BadgeCanisterStatus status={getValue() as CanisterStatus} />
        ),
        header: "Status",
      },
    ],
    []
  );

  const { data, isSuccess, isLoading, isError } = useFetchAllCanisters(
    SNS_ROOT_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );

  //   const handleClickView = (cell: CellContext<any, unknown>) => {
  //     const columnId = cell.column?.id;
  //     const row = cell?.row?.original;
  //     const pathnames = {
  //       index: `/explorer/transactions/${row?.index}`,
  //       to_account: `/explorer/transactions/accounts/${row?.to_account}`,
  //       from_account: `/explorer/transactions/accounts/${row?.from_account}`,
  //     };
  //     navigate(pathnames[columnId]);
  //   };

  return (
    <div>
      {isSuccess && (
        <Table
          columns={columns}
          data={data}
          serverSide={false}
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
