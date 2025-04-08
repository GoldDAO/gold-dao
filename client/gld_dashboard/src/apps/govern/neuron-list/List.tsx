import { useMemo } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { useAuth } from "@auth/index";
import useFetchUserNeuronsList from "@services/sns_governance/hooks/useFetchUserNeuronsList";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import { NeuronUser } from "@services/sns_governance/utils/interfaces";
import { Logo, Table } from "@components/index";
import { BadgeNeuronState } from "@components/badges/BadgeNeuronState";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import {
  GOLDAO_LEDGER_CANISTER_ID,
  SNS_GOVERNANCE_CANISTER_ID,
} from "@constants";

const List = () => {
  const { authenticatedAgent, unauthenticatedAgent, isConnected, principalId } =
    useAuth();

  const { status, data } = useFetchUserNeuronsList(
    SNS_GOVERNANCE_CANISTER_ID,
    authenticatedAgent,
    {
      owner: principalId,
      enabled: !!authenticatedAgent && isConnected && !!principalId,
    }
  );

  const decimals = useFetchDecimals(
    GOLDAO_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "goldao",
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const columns = useMemo<ColumnDef<NeuronUser>[]>(
    () => [
      {
        accessorKey: "staked_amount",
        id: "staked_amount",
        cell: ({ getValue }) => (
          <div className="flex items-center gap-2">
            {!decimals.isSuccess ? (
              <div>Loading...</div>
            ) : (
              <>
                <Logo name="goldao" className="h-4" />
                <TokenValueToLocaleString
                  value={getValue() as bigint}
                  tokenDecimals={decimals.data}
                />
              </>
            )}
          </div>
        ),
        header: "GOLDAO Staked",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "state",
        id: "state",
        cell: ({ getValue }) => (
          <div className="flex items-center justify-center">
            <BadgeNeuronState
              state={
                getValue() as "not dissolving" | "dissolving" | "dissolved"
              }
            />
          </div>
        ),
        header: "State",
      },
      {
        accessorKey: "dissolve_delay",
        id: "dissolve_delay",
        cell: ({ getValue }) => (
          <div className="flex items-center justify-center">
            <div>{getValue() as string}</div>
          </div>
        ),
        header: "Dissolve Delay",
      },
      {
        accessorKey: "age",
        id: "age",
        cell: ({ getValue }) => (
          <div className="flex items-center justify-center">
            <div>{getValue() as string}</div>
          </div>
        ),
        header: "Age",
      },
    ],
    [decimals.isSuccess, decimals.data]
  );

  if (status === "success" && data?.length === 0) {
    return (
      <div className="p-4 flex justify-center border border-border rounded-lg">
        <div>No neurons found</div>
      </div>
    );
  }

  return (
    <div className="bg-surface-primary rounded-xl p-2 border border-border">
      {status === "success" ? (
        <Table
          columns={columns}
          data={data}
          pagination={{
            pageIndex: 0,
            pageSize: 100,
          }}
          serverSide={false}
        />
      ) : (
        <div className="flex justify-center items-center p-4 lg:p-8">
          {status === "pending" && <div>Loading...</div>}
          {status === "error" && <div>Error</div>}
        </div>
      )}
    </div>
  );
};

export default List;
