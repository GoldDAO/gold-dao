import clsx from "clsx";
import { GOLDAO_LEDGER_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import useGetAllNeuronsTotalStakedAmount from "../utils/useGetAllNeuronsTotalStakedAmount";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import GradientCard from "@shared/components/ui/card/GradientCard";

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

  const renderTotalActiveStake = () => {
    if (isConnected) {
      if (tokenPrice.isSuccess) {
        return (
          <>
            <TokenValueToLocaleString
              value={tokenPrice.data.amount}
              tokenDecimals={tokenPrice.data.decimals}
            />
            <div className="text-content/60 font-normal">GOLDAO</div>
          </>
        );
      } else {
        return "Loading...";
      }
    } else {
      return (
        <>
          0<div className="text-content/60 font-normal">GOLDAO</div>
        </>
      );
    }
  };

  const renderTotalActiveStakePrice = () => {
    if (isConnected) {
      if (tokenPrice.isSuccess) {
        return (
          <>
            $<NumberToLocaleString value={tokenPrice.data.amount_usd} />
          </>
        );
      } else {
        return "Loading...";
      }
    } else {
      return <>$0</>;
    }
  };

  return (
    <GradientCard
      className={clsx(
        "px-4 xl:px-8 pt-4 xl:pt-8 pb-24",
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
                  {renderTotalActiveStake()}
                </div>
              </div>
            </div>
            <div className="text-sm text-content/60">
              {renderTotalActiveStakePrice()}
            </div>
          </div>
        </div>
      </div>
    </GradientCard>
  );
};

export default NeuronOverview;
