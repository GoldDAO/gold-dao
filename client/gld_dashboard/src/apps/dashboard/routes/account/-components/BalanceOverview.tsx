import { useNavigate } from "react-router-dom";
import { ArrowsRightLeftIcon } from "@heroicons/react/20/solid";

import { useAuth } from "@auth/index";
import {
  ICP_LEDGER_CANISTER_ID,
  GOLDAO_LEDGER_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";

import { Logo } from "@components/logos";

import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import { Ledger } from "@services/ledger/utils/interfaces";

import { LoaderSpin } from "@components/loaders";
import E8sToLocaleString from "@components/numbers/E8sToLocaleString";

const TokenBalance = ({
  token,
  canisterId,
}: {
  token: Ledger;
  canisterId: string;
}) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();
  const { data, isSuccess, isLoading } = useFetchUserBalance(
    canisterId,
    unauthenticatedAgent,
    {
      ledger: token,
      owner: principalId,
      enabled: !!unauthenticatedAgent && !!isConnected,
    }
  );

  return (
    <div className="flex items-center gap-3">
      {isSuccess && (
        <p className="font-semibold text-2xl">
          <E8sToLocaleString value={data} />
        </p>
      )}
      {isLoading && <LoaderSpin size="sm" />}
      <Logo className="flex-none h-6" name={token} />
    </div>
  );
};

const BalanceOverview = () => {
  const navigate = useNavigate();

  const tokensList: { name: Ledger; label: string; canisterId: string }[] = [
    { name: "gldgov", label: "GLDGov", canisterId: GOLDAO_LEDGER_CANISTER_ID },
    { name: "icp", label: "ICP", canisterId: ICP_LEDGER_CANISTER_ID },
    { name: "ogy", label: "OGY", canisterId: OGY_LEDGER_CANISTER_ID },
    { name: "gldt", label: "GLDT", canisterId: GLDT_LEDGER_CANISTER_ID },
  ];

  const handleNavigateTransfer = (token: Ledger) => {
    navigate(`/dashboard/account/transfer?token=${token}`);
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-4 gap-4">
      {tokensList.map(({ name, label, canisterId }) => (
        <div
          className="border border-border rounded-xl bg-surface-primary p-6 lg:p-8"
          key={name}
        >
          <div className="flex items-center gap-3 mb-4">
            <div className="font-light text-content/60">{label}</div>
          </div>
          <div className="mt-4">
            <div className="flex items-center justify-between">
              <TokenBalance token={name} canisterId={canisterId} />
              <button
                className="flex justify-center items-center p-2 bg-accent-2 text-white rounded-full"
                data-tooltip-id="tooltip"
                data-tooltip-content="Transfer / Send"
                onClick={() => handleNavigateTransfer(name)}
              >
                <ArrowsRightLeftIcon className="h-5" />
              </button>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};

export default BalanceOverview;
