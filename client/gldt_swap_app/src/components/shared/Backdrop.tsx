import { ButtonHTMLAttributes } from "react";

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement>;

const Backdrop = ({ ...restprops }: ButtonProps) => {
  return <button className={`absolute h-full w-full z-50`} {...restprops} />;
};

export default Backdrop;
