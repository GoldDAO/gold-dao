import clsx from "clsx";
import { ReactNode } from "react";

const InnerAppLayout = ({ children }: { children: ReactNode }) => (
  <div className="flex flex-col lg:flex-row w-full rounded-[inherit]">
    {children}
  </div>
);

InnerAppLayout.LeftPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "flex flex-col lg:flex-grow overflow-hidden",
      "lg:bg-surface-primary",
      "lg:border-r lg:border-l-0 border-border",
      "py-4 lg:py-8",
      "lg:max-w-[400px]"
    )}
  >
    {children}
  </div>
);
InnerAppLayout.RightPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "flex flex-col lg:flex-grow overflow-hidden",
      "bg-background rounded-r-[inherit]"
    )}
  >
    {children}
  </div>
);

export default InnerAppLayout;
