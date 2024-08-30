import { ReactNode } from "react";

const ColInfoGLDT = ({
  children,
  className,
}: {
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center gap-2">
        <img className="flex-none h-4" src={`/gldt_logo.svg`} />
        <div className="font-semibold">{children}</div>
        <div className="text-content/60 text-sm">GLDT</div>
      </div>
    </div>
  );
};

export default ColInfoGLDT;
