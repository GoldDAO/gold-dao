import { useMemo } from "react";
import { ColumnDef, Row } from "@tanstack/react-table";
import { ChevronUpIcon, ChevronDownIcon } from "@heroicons/react/20/solid";

import { useAuth } from "@auth/index";
import {
  SNS_GOVERNANCE_CANISTER_ID,
  SNS_REWARDS_CANISTER_ID,
  GOLDAO_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";

import { Table, LoaderSpin, Logo } from "@components/index";
import CopyToClipboard from "@components/buttons/CopyToClipboard";

import { TableProps } from "@utils/table/useTable";

import { NeuronUser } from "@services/sns_governance/hooks/interfaces";
import useFetchUserNeurons from "@services/sns_governance/hooks/useFetchUserNeurons";

import { ClaimOneNeuronRewards } from "./claim-one-neuron-rewards/ClaimOneNeuronRewards";

const NeuronExpandedRow = ({ row }: { row: Row<NeuronUser> }) => {
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

const UserNeuronsTable = ({
  pagination = {
    pageIndex: 0,
    pageSize: 10,
  },
  setPagination,
}: TableProps) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const columns = useMemo<ColumnDef<NeuronUser>[]>(
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
              <div className="flex items-center max-w-sm">
                <div
                  className="mr-2 truncate"
                  data-tooltip-id="tooltip"
                  data-tooltip-content={getValue() as string}
                >
                  {getValue() as string}
                </div>
                <CopyToClipboard value={getValue() as string} />
              </div>
            </div>
          ) : (
            ""
          ),
        header: "",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "claim_balance_gldgov",
        id: "claim_balance_gldgov",
        cell: ({ getValue }) => (
          <div className="flex items-center gap-2">
            <Logo name="gldgov" className="h-5" />
            <div className="font-semibold">{getValue() as string}</div>
            <div className="text-content/60">GLDGov</div>
          </div>
        ),
        header: "",
      },
      {
        accessorKey: "claim_balance_icp",
        id: "claim_balance_icp",
        cell: ({ getValue }) => (
          <div className="flex items-center gap-2">
            <Logo name="icp" className="h-5" />
            <div className="font-semibold">{getValue() as string}</div>
            <div className="text-content/60">ICP</div>
          </div>
        ),
        header: "",
      },
      {
        accessorKey: "claim_balance_ogy",
        id: "claim_balance_ogy",
        cell: ({ getValue }) => (
          <div className="flex items-center gap-2">
            <Logo name="ogy" className="h-5" />
            <div className="font-semibold">{getValue() as string}</div>
            <div className="text-content/60">OGY</div>
          </div>
        ),
        header: "",
      },
      {
        accessorKey: "",
        id: "claim_rewards",
        cell: ({ row }) => (
          <ClaimOneNeuronRewards
            neuronId={row?.original?.id}
            amountGLDGov={row?.original?.claim_balance_gldgov}
            amountICP={row?.original?.claim_balance_icp}
            amountOGY={row?.original?.claim_balance_ogy}
          />
        ),
        header: "",
      },
    ],
    []
  );

  const { data, isSuccess, isLoading, isError } = useFetchUserNeurons(
    SNS_GOVERNANCE_CANISTER_ID,
    unauthenticatedAgent,
    {
      owner: principalId,
      canisterIdSNSRewards: SNS_REWARDS_CANISTER_ID,
      canisterIdLedgerGOLDAO: GOLDAO_LEDGER_CANISTER_ID,
      canisterIdLedgerOGY: OGY_LEDGER_CANISTER_ID,
      enabled: !!unauthenticatedAgent && !!isConnected && !!principalId,
    }
  );

  return (
    <div>
      {isSuccess && (
        <Table
          columns={columns}
          data={data}
          serverSide={false}
          pagination={pagination}
          setPagination={setPagination}
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

export default UserNeuronsTable;
