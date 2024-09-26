type ButtonProps = React.ComponentPropsWithoutRef<"button">;

const Button = ({
  className,
  children,
  disabled = false,
  type = "button",
  ...restProps
}: ButtonProps) => {
  return (
    <button
      type={type}
      disabled={disabled}
      className={`bg-content text-background py-2 px-4 font-semibold disabled:opacity-60 cursor-pointer disabled:cursor-not-allowed rounded-full ${className}`}
      {...restProps}
    >
      {children}
    </button>
  );
};

export default Button;
