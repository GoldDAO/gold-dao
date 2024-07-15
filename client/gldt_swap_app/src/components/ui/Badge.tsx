import { PropsWithChildren } from "react";

interface IBadge
  extends PropsWithChildren<{
    className?: string;
  }> {}

const Badge = ({ children, className, ...restProps }: IBadge) => {
  return (
    <div className={`rounded-full px-0 py-1 ${className}`} {...restProps}>
      {children}
    </div>
  );
};

export default Badge;
