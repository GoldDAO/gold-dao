import { ReactNode } from "react";

const Base = ({
  className,
  children,
  ...restProps
}: {
  className?: string;
  children: ReactNode;
}) => {
  return (
    <div className={className} {...restProps}>
      <div className="border border-border bg-surface-2 py-3 px-4 rounded-xl">
        {children}
      </div>
    </div>
  );
};

export default Base;
