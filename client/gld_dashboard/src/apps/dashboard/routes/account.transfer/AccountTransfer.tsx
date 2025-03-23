import { useSearchParams } from "react-router-dom";

import {
  ICP_LEDGER_CANISTER_ID,
  GOLDAO_LEDGER_CANISTER_ID,
  GLDT_LEDGER_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
} from "@constants";

import Transfer from "apps/dashboard/-components/Transfer";
import { Ledger } from "@services/ledger/utils/interfaces";

const AccountTransfer = () => {
  const [searchParams] = useSearchParams();
  const token = searchParams.get("token");

  const canisters: { [key: string]: string } = {
    gldgov: GOLDAO_LEDGER_CANISTER_ID,
    icp: ICP_LEDGER_CANISTER_ID,
    ogy: OGY_LEDGER_CANISTER_ID,
    gldt: GLDT_LEDGER_CANISTER_ID,
  };

  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Account</div>
        <div className="text-4xl">Transfer</div>
      </div>

      <section className="p-8 max-w-2xl mx-auto bg-surface-primary border border-border rounded-xl">
        <Transfer
          token={token as Ledger}
          canisterId={canisters[token as string]}
        />
      </section>
    </>
  );
};

export default AccountTransfer;
