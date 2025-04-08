import clsx from "clsx";
import { GOLDAO_LEDGER_CANISTER_ID, GLDT_VALUE_1G_NFT } from "@constants";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import useGetTokenTotalStakedAmount from "../utils/useGetTokenTotalStakedAmount";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

const NeuronOverview = () => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();

  const stakedAmount = useGetTokenTotalStakedAmount({
    canisterIdLedger: GOLDAO_LEDGER_CANISTER_ID,
    owner: principalId,
    agent: unauthenticatedAgent,
    enabled: !!unauthenticatedAgent && isConnected && !!principalId,
  });

  const decimals = useFetchDecimals(
    GOLDAO_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: GOLDAO_LEDGER_CANISTER_ID,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  return (
    <div
      className={clsx(
        "bg-linear-to-t from-neutral-100 to-background dark:from-neutral-900 dark:to-neutral-800",
        "rounded-tr-[inherit] rounded-tl-0 px-4 lg:px-8 pt-4 lg:pt-8 pb-24"
      )}
    >
      <div className="flex flex-col items-center">
        <div className="flex flex-col gap-2 items-center">
          <div className="flex items-center gap-2">
            <Logo name="goldao" className="h-8 w-8" />
            <div>GOLDAO</div>
          </div>
        </div>
        <div className="py-8">
          <div className="flex flex-col items-center gap-2">
            <div>
              <div className="font-semibold flex flex-col items-center gap-2">
                <div>Total active stake</div>
                <div className="text-2xl lg:text-4xl  flex items-center gap-2">
                  {stakedAmount.isSuccess && decimals.isSuccess ? (
                    <>
                      <TokenValueToLocaleString
                        value={stakedAmount.data}
                        tokenDecimals={decimals.data}
                      />
                      <div className="text-content/60 font-normal">GOLDAO</div>
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
              </div>
            </div>
            <div>
              {stakedAmount.isSuccess && decimals.isSuccess ? (
                <div>
                  <TokenValueToLocaleString
                    value={stakedAmount.data / BigInt(GLDT_VALUE_1G_NFT)}
                    tokenDecimals={decimals.data}
                  />{" "}
                  grams of Gold ($todo)
                </div>
              ) : (
                <div>Loading...</div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NeuronOverview;
