import type { ButtonHTMLAttributes, ReactNode } from "react";
import { Button } from "@components/index";
import clsx from "clsx";

type Size = "sm" | "md" | "lg";
type Variant = "filled" | "outlined";
type Shape = "square" | "round";

const sizeClasses: Record<Size, string> = {
  sm: "px-4 py-1 text-sm",
  md: "px-6 py-3 text-base",
  lg: "px-6 py-4 text-lg",
};

const variantClasses: Record<Variant, string> = {
  filled:
    "bg-primary text-white font-semibold hover:bg-primary/80 disabled:bg-primary/60",
  outlined:
    "bg-transparent border-1 border-primary text-primary font-semibold hover:bg-primary/10 disabled:border-primary/60 disabled:text-primary/60",
};

const shapeClasses: Record<Shape, string> = {
  square: "rounded-xl",
  round: "rounded-full",
};

type BtnPrimaryProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  children: ReactNode;
  size?: Size;
  variant?: Variant;
  shape?: Shape;
  className?: string;
};

const BtnPrimary = ({
  children,
  size = "md",
  variant = "filled",
  shape = "square",
  className,
  ...props
}: BtnPrimaryProps) => {
  return (
    <Button
      className={clsx(
        sizeClasses[size],
        variantClasses[variant],
        shapeClasses[shape],
        className
      )}
      {...props}
    >
      {children}
    </Button>
  );
};

export default BtnPrimary;
