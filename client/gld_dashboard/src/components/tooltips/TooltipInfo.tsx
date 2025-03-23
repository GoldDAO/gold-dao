import { PropsWithChildren } from "react";
import { InformationCircleIcon } from "@heroicons/react/24/outline";
import Tooltip from "./Tooltip";

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
      <InformationCircleIcon
        className="h-6 w-6 text-content/60 cursor-pointer"
        data-tooltip-id={id}
      />
      <Tooltip id={id} clickable={clickable} className="max-w-64">
        {children}
      </Tooltip>
    </div>
  );
};

export default TooltipInfo;
