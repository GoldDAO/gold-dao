import { ReactNode } from "react";

const Card = ({
  className,
  children,
  ...restProps
}: {
  className?: string;
  children: ReactNode;
}) => {
  return (
    <div
      className={`relative bg-surface border border-border p-6 rounded-xl ${className}`}
      {...restProps}
    >
      {children}
    </div>
  );
};

export default Card;
