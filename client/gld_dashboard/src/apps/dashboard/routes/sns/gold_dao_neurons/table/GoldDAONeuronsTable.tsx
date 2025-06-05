import { useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { ColumnDef, Row } from "@tanstack/react-table";
import { ChevronUpIcon, ChevronDownIcon } from "@heroicons/react/20/solid";

import { useAuth } from "@auth/index";
import {
  SNS_GOVERNANCE_CANISTER_ID_IC,
  SNS_ROOT_CANISTER_ID,
} from "@constants";

import { Table, LoaderSpin } from "@components/index";
import { BadgeNeuronState } from "@components/badges/BadgeNeuronState";
import CopyToClipboard from "@components/buttons/CopyToClipboard";

import { TableProps } from "@shared/utils/table/useTable";

import { NeuronPartial } from "@services/sns_governance/hooks/interfaces";
import useFetchAllNeurons from "@services/sns_governance/hooks/useFetchAllNeurons";

const NeuronExpandedRow = ({ row }: { row: Row<NeuronPartial> }) => {
  const neuron = row?.original ?? undefined;
  if (!neuron) return null;

  const data = [
    { label: "Date Created", value: neuron.created_at },
    {
      label: "Auto-Stake Maturity",
      value: neuron.auto_stake_maturity,
    },
    ...(neuron.is_voting_power
      ? [
          {
            label: "Dissolve Delay Bonus",
            value: neuron.dissolve_delay_bonus,
          },
          { label: "Age Bonus", value: neuron.age_bonus },
          { label: "Total Bonus", value: neuron.total_bonus },
        ]
      : []),
  ];

  return (
    <div className="flex border-b border-border">
      {data.map(({ label, value }) => (
        <div key={label} className="py-4 px-6 text-sm">
          <div className="text-content/60">{label}</div>
          <div className="font-semibold">{value}</div>
        </div>
      ))}
    </div>
  );
};

export const GoldDAONeuronsTable = ({
  pagination = {
    pageIndex: 0,
    pageSize: 10,
  },
  setPagination,
  sorting,
  setSorting,
}: TableProps) => {
  const navigate = useNavigate();

  const { unauthenticatedAgent } = useAuth();

  const columns = useMemo<ColumnDef<NeuronPartial>[]>(
    () => [
      {
        accessorKey: "id",
        id: "id",
        cell: ({ row, getValue }) =>
          row.getCanExpand() ? (
            <div className="flex items-center">
              <button
                {...{
                  onClick: row.getToggleExpandedHandler(),
                }}
                className="cursor-pointer mr-2"
              >
                {row.getIsExpanded() ? (
                  <ChevronUpIcon className="h-5 w-5" />
                ) : (
                  <ChevronDownIcon className="h-5 w-5" />
                )}
              </button>
              <div className="flex items-center max-w-sm text-accent">
                <button
                  className="mr-2 truncate"
                  data-tooltip-id="tooltip"
                  data-tooltip-content={getValue() as string}
                  onClick={() => handleOnClickNeuron(getValue() as string)}
                >
                  {getValue() as string}
                </button>
                <CopyToClipboard value={getValue() as string} />
              </div>
            </div>
          ) : (
            ""
          ),
        header: "ID",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "state",
        id: "state",
        cell: ({ getValue }) => (
          <BadgeNeuronState
            state={getValue() as "not dissolving" | "dissolving" | "dissolved"}
          />
        ),
        header: "State",
      },
      {
        accessorKey: "staked_amount",
        id: "staked_amount",
        cell: ({ getValue }) => (
          <div>
            <span>{getValue() as string}</span>
            <span className="text-content/60 text-sm"> GLDGov</span>
          </div>
        ),
        header: "Staked GLDGov",
      },
      {
        accessorKey: "total_maturity",
        id: "total_maturity",
        cell: ({ getValue }) => getValue() as string,
        header: "Maturity",
      },
      {
        accessorKey: "dissolve_delay",
        id: "dissolve_delay",
        cell: ({ getValue }) => getValue() as string,
        header: "Dissolve Delay",
      },
      {
        accessorKey: "age",
        id: "age",
        cell: ({ getValue }) => getValue() as string,
        header: "Age",
      },
      {
        accessorKey: "voting_power",
        id: "voting_power",
        cell: ({ getValue }) => getValue() as string,
        header: "Voting Power",
      },
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  const handleOnClickNeuron = (neuronId: string) => {
    navigate(`/dashboard/sns/neurons/${neuronId}`);
  };

  const { data, isSuccess, isLoading, isError } = useFetchAllNeurons(
    SNS_GOVERNANCE_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      limit: pagination.pageSize,
      offset: pagination.pageSize * pagination.pageIndex,
      sorting,
      snsRootCanisterId: SNS_ROOT_CANISTER_ID,
      enabled: !!unauthenticatedAgent,
    }
  );

  return (
    <div>
      {isSuccess && (
        <Table
          columns={columns}
          data={data.data}
          rowCount={data.total_neurons}
          pagination={pagination}
          setPagination={setPagination}
          sorting={sorting}
          setSorting={setSorting}
          getRowCanExpand={() => true}
          rowExpanded={NeuronExpandedRow}
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
