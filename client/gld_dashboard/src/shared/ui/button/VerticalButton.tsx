import { ReactNode, ButtonHTMLAttributes } from "react";
import clsx from "clsx";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
  icon: ReactNode;
}

const VerticalButton = ({ icon, children, ...props }: ButtonProps) => {
  return (
    <button
      className={clsx(
        "rounded-xl shrink-0 cursor-pointer disabled:cursor-default",
        "bg-primary",
        "w-[64px] xl:w-[140px]"
      )}
      {...props}
    >
      <div
        className={clsx(
          "flex flex-col justify-center items-center gap-1",
          "px-1 py-2 md:py-3 rounded-[inherit]",
          "bg-primary text-white",
          {
            "bg-primary/60": props.disabled,
            "hover:bg-primary/80": !props.disabled,
          }
        )}
      >
        {icon}
        <div className="text-xs xl:text-sm">{children}</div>
      </div>
    </button>
  );
};

export default VerticalButton;
