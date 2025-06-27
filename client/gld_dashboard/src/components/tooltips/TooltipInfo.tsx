import { PropsWithChildren } from "react";
import Tooltip from "./Tooltip";
import Icon from "@shared/ui/icons";

interface TooltipInfoProps extends PropsWithChildren {
  className?: string;
  id?: string;
  clickable?: boolean;
  openOnClick?: boolean;
}

const TooltipInfo = ({
  children,
  id = "tootltip-id",
  clickable = false,
}: TooltipInfoProps) => {
  return (
    <div>
      <Icon.InfoCircle
        width={18}
        height={18}
        className="text-content/60 cursor-pointer"
        data-tooltip-id={id}
      />
      <Tooltip id={id} clickable={clickable} className="max-w-64">
        {children}
      </Tooltip>
    </div>
  );
};

export default TooltipInfo;
