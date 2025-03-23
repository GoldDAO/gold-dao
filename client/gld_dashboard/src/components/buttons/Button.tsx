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
      className={`disabled:opacity-70 cursor-pointer disabled:cursor-not-allowed ${className}`}
      {...restProps}
    >
      {children}
    </button>
  );
};

export default Button;
