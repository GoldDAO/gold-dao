import clsx from "clsx";
import { GOLDAO_LEDGER_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import useGetAllNeuronsTotalStakedAmount from "../utils/useGetAllNeuronsTotalStakedAmount";
import useFetchTokenPrice from "@hooks/useFetchTokenPrice";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

const NeuronOverview = () => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();

  const stakedAmount = useGetAllNeuronsTotalStakedAmount({
    owner: principalId,
    agent: unauthenticatedAgent,
    enabled: !!unauthenticatedAgent && isConnected && !!principalId,
  });

  const tokenPrice = useFetchTokenPrice(unauthenticatedAgent, {
    from: "GOLDAO",
    from_canister_id: GOLDAO_LEDGER_CANISTER_ID,
    amount: stakedAmount.data ?? 0n,
    enabled: !!unauthenticatedAgent && isConnected && stakedAmount.isSuccess,
  });

  return (
    <div
      className={clsx(
        "bg-linear-to-t from-neutral-100 to-background dark:from-neutral-900 dark:to-neutral-800",
        "rounded-tr-[inherit] rounded-tl-0 px-4 xl:px-8 pt-4 xl:pt-8 pb-24"
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
                <div className="text-2xl xl:text-4xl  flex items-center gap-2">
                  {tokenPrice.isSuccess ? (
                    <>
                      <TokenValueToLocaleString
                        value={tokenPrice.data.amount}
                        tokenDecimals={tokenPrice.data.decimals}
                      />
                      <div className="text-content/60 font-normal">GOLDAO</div>
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
              </div>
            </div>
            <div className="text-lg text-content/60">
              {tokenPrice.isSuccess ? (
                <div>
                  ($
                  <NumberToLocaleString value={tokenPrice.data.amount_usd} />)
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
