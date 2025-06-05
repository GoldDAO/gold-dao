import clsx from "clsx";
import { ReactNode } from "react";

const GradientCard = ({
  className,
  children,
}: {
  className?: string;
  children?: ReactNode;
}) => (
  <div
    className={clsx(
      "bg-linear-to-t from-neutral-100 to-background dark:from-neutral-900 dark:to-neutral-800",
      className
    )}
  >
    {children}
  </div>
);

export default GradientCard;
