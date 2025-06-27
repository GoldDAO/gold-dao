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
      "xl:bg-linear-to-t xl:from-neutral-100 xl:to-background xl:dark:from-neutral-900 xl:dark:to-neutral-800",
      className
    )}
  >
    {children}
  </div>
);

export default GradientCard;
