import { PropsWithChildren } from "react";

type TileProps = PropsWithChildren<{
  className?: string;
}>;

const Tile = ({ className, children, ...restProps }: TileProps) => {
  return (
    <div
      className={`flex justify-center items-center shrink-0 w-12 h-12 rounded-full ${className}`}
      {...restProps}
    >
      {children}
    </div>
  );
};

export default Tile;
