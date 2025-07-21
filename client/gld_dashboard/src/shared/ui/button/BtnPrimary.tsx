import type { ButtonHTMLAttributes, ReactNode } from "react";
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
    "bg-primary text-white hover:bg-primary/80 disabled:bg-primary/60 disabled:cursor-not-allowed",
  outlined:
    "bg-transparent border-1 border-primary text-primary hover:bg-primary/10 disabled:border-primary/60 disabled:text-primary/60 disabled:cursor-not-allowed",
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
  const shapeClass =
    size === "sm" && shape === "square" ? "rounded-md" : shapeClasses[shape];
  return (
    <button
      className={clsx(
        "cursor-pointer",
        sizeClasses[size],
        variantClasses[variant],
        shapeClass,
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
};

export default BtnPrimary;
