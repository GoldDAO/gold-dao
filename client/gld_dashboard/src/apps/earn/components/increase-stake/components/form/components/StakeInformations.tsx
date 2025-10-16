import { ReactNode } from "react";
import { useAuth } from "@auth/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import useFetchStakeAPY from "@earn/hooks/useFetchStakeAPY";
import { Logo } from "@components/index";
import Icon from "@shared/ui/icons";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const StakeInformations = ({ className }: { className?: string }) => {
  const { unauthenticatedAgent } = useAuth();

  const stakeAPY = useFetchStakeAPY(
    GLDT_STAKE_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );

  const infos: Array<{ title: string; subtitle: string; icon: ReactNode }> = [
    {
      title: "Staking rewards will be paid in tokens GOLDAO.",
      subtitle: "",
      icon: <Logo name="gldt" className="w-8" />,
    },
    {
      title: "Current APY",
      subtitle:
        "Based on last weeks APY. You will be part of next round of payment.",
      icon: <Icon.Insights size={32} />,
    },
    {
      title: "Unlock delay of 1 week",
      subtitle:
        "When unlocking GLDT from staking, the tokens are locked for 1 week without rewards before they can be withdrawn.",
      icon: <Icon.Clock width={32} />,
    },
    {
      title: "Age bonus",
      subtitle:
        "GLDT stakes start obtaining an age bonus from day 1. The older the stakes, the bigger the age bonus, growing linearly at 100% per year.",
      icon: <Icon.SquareStack width={32} />,
    },
    {
      title:
        "When you start unlocking your GLDT stake, you will no longer receive new rewards.",
      subtitle: "",
      icon: <Icon.DoNotDisturb size={32} />,
    },
  ];
  return (
    <div className={className}>
      <div className="flex flex-col gap-2">
        {infos.map((info, index) => (
          <div
            className="flex flex-col xl:flex-row gap-2 xl:gap-0 items-center border rounded-lg p-4 border-border"
            key={index}
          >
            <div className="pr-4 text-gold">{info.icon}</div>
            <div className="flex flex-col items-center xl:items-start">
              <div className="flex items-center gap-1 text-sm font-semibold text-content/80 text-center xl:text-start">
                {info.title}{" "}
                {index === 1 && (
                  <div>
                    {stakeAPY.isSuccess ? (
                      <>
                        <NumberToLocaleString value={stakeAPY.data} />%
                      </>
                    ) : (
                      <div className="animate-pulse">0%</div>
                    )}
                  </div>
                )}
              </div>
              <div className="text-sm text-content/60 text-center xl:text-start">
                {info.subtitle}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default StakeInformations;
