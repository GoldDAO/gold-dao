import clsx from "clsx";
import { ReactNode } from "react";

const InnerAppLayout = ({ children }: { children: ReactNode }) => (
  <div className="flex flex-col xl:flex-row w-full rounded-[inherit]">
    {children}
  </div>
);

InnerAppLayout.LeftPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "flex flex-col xl:flex-grow overflow-hidden",
      "xl:bg-surface-primary",
      "xl:border-r xl:border-l-0 border-border",
      "py-4 xl:py-8",
      "xl:max-w-[400px]"
    )}
  >
    {children}
  </div>
);
InnerAppLayout.RightPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "flex flex-col xl:flex-grow overflow-hidden",
      "bg-background rounded-r-[inherit]"
    )}
  >
    {children}
  </div>
);

export default InnerAppLayout;
