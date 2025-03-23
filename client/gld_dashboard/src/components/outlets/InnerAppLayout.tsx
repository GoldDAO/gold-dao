import clsx from "clsx";
import { ReactNode } from "react";

const InnerAppLayout = ({ children }: { children: ReactNode }) => (
  <div className="lg:grid lg:grid-cols-3 w-full rounded-[inherit]">
    {children}
  </div>
);

InnerAppLayout.LeftPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "lg:bg-surface-primary flex flex-col lg:flex-grow",
      "lg:border-r border-border",
      "px-4 lg:px-8 py-4 lg:py-8"
    )}
  >
    {children}
  </div>
);
InnerAppLayout.RightPanel = ({ children }: { children: ReactNode }) => (
  <div
    className={clsx(
      "bg-background lg:col-span-2 rounded-[inherit] flex flex-col lg:flex-grow"
    )}
  >
    {children}
  </div>
);

export default InnerAppLayout;
