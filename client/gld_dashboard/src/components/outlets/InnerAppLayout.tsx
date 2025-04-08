import clsx from "clsx";
import { ReactNode } from "react";

const InnerAppLayout = ({ children }: { children: ReactNode }) => (
  <div className="grid grid-cols-1 lg:grid-cols-3 w-full rounded-[inherit]">
    {children}
  </div>
);

InnerAppLayout.LeftPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "flex flex-col lg:flex-grow overflow-hiden",
      "lg:bg-surface-primary",
      "lg:border-r lg:border-l-0 border-border",
      "py-4 lg:py-8"
    )}
  >
    {children}
  </div>
);
InnerAppLayout.RightPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "flex flex-col lg:flex-grow overflow-hidden",
      "bg-background lg:col-span-2 rounded-r-[inherit]"
    )}
  >
    {children}
  </div>
);

export default InnerAppLayout;
