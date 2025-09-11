import { useAuth } from "@auth/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import useFetchStakeAPY from "@earn/hooks/useFetchStakeAPY";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import Icon from "@shared/ui/icons";

const StakeAPY = () => {
  const { unauthenticatedAgent } = useAuth();

  const stakeAPY = useFetchStakeAPY(
    GLDT_STAKE_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );

  return (
    <div className="border rounded-xl p-4 border-border">
      <div className="flex flex-col items-center xl:items-start gap-1">
        <div className="flex items-center gap-1 text-content/60">
          <div>Current APY</div>
          <button
            data-tooltip-id="tooltip"
            data-tooltip-html={
              "APY for staking GLDT based on current token prices"
            }
          >
            <Icon.InfoCircle width={16} />
          </button>
        </div>
        <div className="text-2xl font-semibold">
          {stakeAPY.isSuccess ? (
            <>
              <NumberToLocaleString value={stakeAPY.data} decimals={1} />%
            </>
          ) : (
            <div className="animate-pulse">0%</div>
          )}
        </div>
      </div>
    </div>
  );
};

export default StakeAPY;
