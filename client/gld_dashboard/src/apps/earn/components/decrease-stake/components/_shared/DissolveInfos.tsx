import { ReactNode } from "react";
import Icon from "@shared/ui/icons";
import clsx from "clsx";

const DissolveInformations = ({
  className,
  checked = false,
}: {
  className?: string;
  checked?: boolean;
}) => {
  const infos: Array<{ title: string; subtitle: string; icon: ReactNode }> = [
    {
      title: "Unlock delay of 1 week",
      subtitle:
        "When unlocking GLDT from staking, the tokens are locked for 1 week without rewards before they can be withdrawn.",
      icon: <Icon.Clock width={32} />,
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
            className={clsx(
              "flex flex-col xl:flex-row gap-2 xl:gap-0 items-center border rounded-lg p-4 border-border",
              {
                "bg-surface-primary border-gold/30": checked,
              }
            )}
            key={index}
          >
            <div className="pr-4 text-gold">{info.icon}</div>
            <div className="flex flex-col items-center xl:items-start">
              <div className="flex items-center gap-1 text-sm font-semibold text-content/80 text-center xl:text-start">
                {info.title}
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

export default DissolveInformations;
