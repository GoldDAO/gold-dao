import { GLDT_VALUE_1G_NFT, GLDT_LEDGER_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

const TotalCountGLDT = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();

  const balance = useFetchLedgerBalance(
    GLDT_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "gldt",
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  return (
    <div className="flex flex-col items-center gap-2">
      <div>
        {balance.isSuccess ? (
          <div className="text-2xl xl:text-4xl font-semibold flex items-center gap-2">
            <NumberToLocaleString value={balance.data.balance} />
            <div className="text-content/60 font-normal">GLDT</div>
          </div>
        ) : (
          <div>Loading...</div>
        )}
      </div>
      <div className="text-sm text-content/60">
        {balance.isSuccess ? (
          <div>
            <NumberToLocaleString
              value={balance.data.balance / GLDT_VALUE_1G_NFT}
            />{" "}
            grams of Gold ({" "}
            <span>
              $
              <NumberToLocaleString value={balance.data.balance_usd} />
            </span>
            )
          </div>
        ) : (
          <div>Loading...</div>
        )}
      </div>
    </div>
  );
};

export default TotalCountGLDT;
