import { forwardRef } from "react";

const Input = forwardRef<
  HTMLInputElement,
  {
    className: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    [x: string]: any;
  }
>(({ className, ...restProps }, ref) => {
  const { errors, ...inputProps } = restProps;

  return (
    <>
      <input
        className={`form-input outline-none focus:outline-none focus:border-border focus:ring-0 ${className}`}
        {...inputProps}
        ref={ref}
      />
      {errors && (
        <em className="text-dark-orange text-sm p-2">{errors?.message}</em>
      )}
    </>
  );
});

export default Input;
