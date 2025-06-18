import clsx from "clsx";
import { ReactNode } from "react";

const InnerAppLayout = ({ children }: { children: ReactNode }) => (
  <div className="flex flex-col xl:flex-row w-full rounded-[inherit] rounded-l-none overflow-x-hidden">
    {children}
  </div>
);

InnerAppLayout.LeftPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "flex flex-col xl:flex-grow w-full xl:overflow-y-auto",
      "xl:bg-surface-primary",
      "xl:border-r xl:border-l-0 border-border",
      "p-4 xl:p-8",
      "xl:max-w-[400px]"
    )}
  >
    {children}
  </div>
);
InnerAppLayout.RightPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "w-full xl:overflow-y-auto",
      "bg-background rounded-r-[inherit]"
    )}
  >
    {children}
  </div>
);

export default InnerAppLayout;
