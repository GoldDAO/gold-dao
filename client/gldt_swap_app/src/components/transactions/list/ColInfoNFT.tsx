import { ReactNode } from "react";

const ColInfoNFT = ({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center gap-2">
        <img className="flex-none h-4" src={`/gold-bars/${children}g.svg`} />
        <div className="font-semibold">{children}</div>
        <span className="text-content/60 text-sm">g of GOLD</span>
      </div>
    </div>
  );
};

export default ColInfoNFT;
