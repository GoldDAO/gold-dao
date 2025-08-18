import clsx from "clsx";
import { ReactNode } from "react";

const OptionBox = ({
  children,
  className = "",
  checked = false,
}: {
  children?: ReactNode;
  className?: string;
  checked?: boolean;
}) => {
  return (
    <div className={className}>
      <div
        className={clsx("p-4 rounded-xl border border-border", {
          "bg-gold/5 border-gold": checked,
        })}
      >
        {children}
      </div>
    </div>
  );
};

export default OptionBox;
